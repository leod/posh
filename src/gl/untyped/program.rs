use std::{collections::BTreeSet, rc::Rc};

use glow::HasContext;

use crate::gl::CreateProgramError;

use super::{Buffer, GeometryStream, VertexAttributeLayout, VertexInfo};

pub struct SamplerInfo {
    /// The name of the sampler uniform.
    pub name: String,

    /// The texture unit to which this sampler is to be bound in the program.
    pub texture_unit: usize,
}

pub struct UniformBlockInfo {
    /// The name of the uniform block.
    pub name: String,

    /// The location to which this uniform block is to be bound in the program.
    pub location: usize,
}

#[derive(Default)]
pub struct ProgramDef {
    pub uniform_block_infos: Vec<UniformBlockInfo>,
    pub sampler_infos: Vec<SamplerInfo>,
    pub vertex_infos: Vec<VertexInfo>,
    pub vertex_shader_source: String,
    pub fragment_shader_source: String,
}

struct ProgramShared {
    gl: Rc<glow::Context>,
    def: ProgramDef,
    id: glow::Program,
}

#[derive(Clone)]
pub struct Program {
    shared: Rc<ProgramShared>,
}

impl Program {
    /// # Panics
    ///
    /// Panics if the `def` contains duplicate texture unit bindings or
    /// duplicate uniform block bindings.
    pub(crate) fn new(gl: Rc<glow::Context>, def: ProgramDef) -> Result<Self, CreateProgramError> {
        def.validate();

        let shared = Rc::new(ProgramShared {
            gl: gl.clone(),
            def,
            id: unsafe { gl.create_program() }.map_err(CreateProgramError::CreateProgram)?,
        });

        // Compile and attach shaders.
        let vertex_shader = Shader::new(
            gl.clone(),
            glow::VERTEX_SHADER,
            &shared.def.vertex_shader_source,
        )?
        .attach(shared.id);

        let fragment_shader = Shader::new(
            gl.clone(),
            glow::FRAGMENT_SHADER,
            &shared.def.fragment_shader_source,
        )?
        .attach(shared.id);

        // Bind vertex attributes. This needs to be done before linking the
        // program.
        {
            let mut index = 0;

            for vertex_info in &shared.def.vertex_infos {
                for attribute in &vertex_info.attributes {
                    unsafe {
                        gl.bind_attrib_location(
                            shared.id,
                            u32::try_from(index).unwrap(),
                            &attribute.name,
                        );
                    }

                    let attribute_info = VertexAttributeLayout::new(&attribute.ty);

                    // Some attributes (e.g. matrices) take up multiple
                    // locations. We only need to bind to the first location,
                    // though.
                    index += attribute_info.num_locations;
                }
            }
        }

        // Link the program.
        let link_status = unsafe {
            gl.link_program(shared.id);

            // Note that we do not check shader compile status before checking
            // the program link status, since this would break pipelining and
            // thereby potentially slow down compilation.
            gl.get_program_link_status(shared.id)
        };

        if !link_status {
            let vertex_shader_info = unsafe { gl.get_shader_info_log(vertex_shader.shader.id) };
            let fragment_shader_info = unsafe { gl.get_shader_info_log(fragment_shader.shader.id) };
            let program_info = unsafe { gl.get_program_info_log(shared.id) };

            return Err(CreateProgramError::CompilationError {
                vertex_shader_info,
                fragment_shader_info,
                program_info,
            });
        }

        // Set texture units.
        for info in &shared.def.sampler_infos {
            let location = unsafe { gl.get_uniform_location(shared.id, &info.name) };

            // We silently ignore location lookup failures here, since program
            // linking is allowed to remove uniforms that are not used by the
            // program.
            if let Some(location) = location {
                unsafe {
                    gl.uniform_1_i32(Some(&location), i32::try_from(info.texture_unit).unwrap());
                }
            }
        }

        // Set uniform block locations.
        for info in &shared.def.uniform_block_infos {
            let index = unsafe { gl.get_uniform_block_index(shared.id, &info.name) };

            // As with texture units, we silently ignore uniform block index
            // lookup failures here.
            if let Some(index) = index {
                unsafe {
                    gl.uniform_block_binding(
                        shared.id,
                        index,
                        u32::try_from(info.location).unwrap(),
                    );
                }
            }
        }

        Ok(Program { shared })
    }

    /// # Panics
    ///
    /// Panics if any of the supplied objects do not belong to the same
    /// `glow::Context`, or if the wrong number of uniform buffers is supplied.
    ///
    /// # Safety
    ///
    /// TODO
    pub unsafe fn draw(&self, uniform_buffers: &[Buffer], geometry: GeometryStream) {
        let shared = &self.shared;
        let gl = &shared.gl;

        assert_eq!(uniform_buffers.len(), shared.def.uniform_block_infos.len());
        for buffer in uniform_buffers {
            assert!(Rc::ptr_eq(buffer.gl(), gl));
        }
        assert!(Rc::ptr_eq(geometry.gl(), gl));

        unsafe {
            gl.use_program(Some(shared.id));
        }

        for (buffer, block_info) in uniform_buffers.iter().zip(&shared.def.uniform_block_infos) {
            let location = u32::try_from(block_info.location).unwrap();

            unsafe {
                gl.bind_buffer_base(glow::UNIFORM_BUFFER, location, Some(buffer.id()));
            }
        }

        geometry.draw();

        // TODO: Remove overly conservative unbinding.
        for (_, block_info) in uniform_buffers.iter().zip(&shared.def.uniform_block_infos) {
            let location = u32::try_from(block_info.location).unwrap();

            unsafe {
                gl.bind_buffer_base(glow::UNIFORM_BUFFER, location, None);
            }
        }

        gl.bind_buffer(glow::UNIFORM_BUFFER, None);

        unsafe {
            gl.use_program(None);
        }
    }
}

impl Drop for ProgramShared {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_program(self.id);
        }
    }
}

struct Shader {
    gl: Rc<glow::Context>,
    id: glow::Shader,
}

impl Shader {
    fn new(gl: Rc<glow::Context>, ty: u32, source: &str) -> Result<Self, CreateProgramError> {
        let id = unsafe { gl.create_shader(ty) }.map_err(CreateProgramError::CreateShader)?;

        unsafe {
            gl.shader_source(id, source);
            gl.compile_shader(id);
        }

        Ok(Self { gl, id })
    }

    fn attach(self, program_id: glow::Program) -> AttachedShader {
        unsafe {
            self.gl.attach_shader(program_id, self.id);
        }

        AttachedShader {
            shader: self,
            program_id,
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_shader(self.id);
        }
    }
}

struct AttachedShader {
    shader: Shader,
    program_id: glow::Program,
}

impl Drop for AttachedShader {
    fn drop(&mut self) {
        unsafe {
            self.shader
                .gl
                .detach_shader(self.program_id, self.shader.id);
        }
    }
}

impl ProgramDef {
    fn validate(&self) {
        {
            let mut names: BTreeSet<_> = BTreeSet::new();

            for info in &self.sampler_infos {
                if names.contains(&info.name) {
                    panic!("Duplicate sampler name: {}", info.name);
                }

                names.insert(info.name.clone());
            }
        }

        {
            let mut texture_units: BTreeSet<_> = BTreeSet::new();

            for info in &self.sampler_infos {
                if texture_units.contains(&info.texture_unit) {
                    panic!("Duplicate sampler texture unit: {}", info.texture_unit);
                }

                texture_units.insert(info.texture_unit);
            }
        }

        {
            let mut names: BTreeSet<_> = BTreeSet::new();

            for info in &self.uniform_block_infos {
                if names.contains(&info.name) {
                    panic!("Duplicate uniform block name: {}", info.name);
                }

                names.insert(info.name.clone());
            }
        }

        {
            let mut locations: BTreeSet<_> = BTreeSet::new();

            for info in &self.uniform_block_infos {
                if locations.contains(&info.location) {
                    panic!("Duplicate uniform block location: {}", info.location);
                }

                locations.insert(info.location);
            }
        }
    }
}
