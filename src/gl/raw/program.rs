use std::{collections::BTreeSet, rc::Rc};

use glow::HasContext;

use crate::{gl::ProgramError, program_def::ProgramDef};

use super::{vertex_layout::VertexAttributeLayout, Buffer, VertexArrayBinding};

struct ProgramShared {
    gl: Rc<glow::Context>,
    def: ProgramDef,
    id: glow::Program,
}

pub struct Program {
    shared: Rc<ProgramShared>,
}

impl Program {
    /// # Panics
    ///
    /// Panics if the `def` contains duplicate texture unit bindings or
    /// duplicate uniform block bindings.
    pub(super) fn new(gl: Rc<glow::Context>, def: ProgramDef) -> Result<Self, ProgramError> {
        validate_program_def(&def);

        let shared = Rc::new(ProgramShared {
            gl: gl.clone(),
            def,
            id: unsafe { gl.create_program() }.map_err(ProgramError::ProgramCreation)?,
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

            for vertex_info in &shared.def.vertex_defs {
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

            return Err(ProgramError::Compiler {
                vertex_shader_info,
                fragment_shader_info,
                program_info,
            });
        }

        // Set texture units.
        for sampler_def in &shared.def.sampler_defs {
            let location = unsafe { gl.get_uniform_location(shared.id, &sampler_def.name) };

            // We silently ignore location lookup failures here, since program
            // linking is allowed to remove uniforms that are not used by the
            // program.
            if let Some(location) = location {
                unsafe {
                    gl.uniform_1_i32(
                        Some(&location),
                        i32::try_from(sampler_def.texture_unit).unwrap(),
                    );
                }
            }
        }

        // Set uniform block locations.
        for uniform_def in &shared.def.uniform_defs {
            let index = unsafe { gl.get_uniform_block_index(shared.id, &uniform_def.block_name) };

            // As with texture units, we silently ignore uniform block index
            // lookup failures here.
            if let Some(index) = index {
                unsafe {
                    gl.uniform_block_binding(
                        shared.id,
                        index,
                        u32::try_from(uniform_def.location).unwrap(),
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
    pub unsafe fn draw(&self, uniform_buffers: &[&Buffer], vertices: VertexArrayBinding) {
        let shared = &self.shared;
        let gl = &shared.gl;

        unsafe {
            gl.use_program(Some(shared.id));
        }

        assert_eq!(uniform_buffers.len(), shared.def.uniform_defs.len());

        for (buffer, block_info) in uniform_buffers.iter().zip(&shared.def.uniform_defs) {
            assert!(Rc::ptr_eq(buffer.gl(), gl));

            let location = u32::try_from(block_info.location).unwrap();

            unsafe {
                gl.bind_buffer_base(glow::UNIFORM_BUFFER, location, Some(buffer.id()));
            }
        }

        assert!(Rc::ptr_eq(vertices.gl(), gl));
        vertices.draw();

        // TODO: Remove overly conservative unbinding.
        for (_, block_info) in uniform_buffers.iter().zip(&shared.def.uniform_defs) {
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
    fn new(gl: Rc<glow::Context>, ty: u32, source: &str) -> Result<Self, ProgramError> {
        let id = unsafe { gl.create_shader(ty) }.map_err(ProgramError::ShaderCreation)?;

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

fn validate_program_def(def: &ProgramDef) {
    {
        let mut names: BTreeSet<_> = BTreeSet::new();

        for info in &def.sampler_defs {
            if names.contains(&info.name) {
                panic!("Duplicate sampler name: {}", info.name);
            }

            names.insert(info.name.clone());
        }
    }

    {
        let mut texture_units: BTreeSet<_> = BTreeSet::new();

        for sampler_def in &def.sampler_defs {
            if texture_units.contains(&sampler_def.texture_unit) {
                panic!(
                    "Duplicate sampler texture unit: {}",
                    sampler_def.texture_unit
                );
            }

            texture_units.insert(sampler_def.texture_unit);
        }
    }

    {
        let mut names: BTreeSet<_> = BTreeSet::new();

        for info in &def.uniform_defs {
            if names.contains(&info.block_name) {
                panic!("Duplicate uniform block name: {}", info.block_name);
            }

            names.insert(info.block_name.clone());
        }
    }

    {
        let mut locations: BTreeSet<_> = BTreeSet::new();

        for info in &def.uniform_defs {
            if locations.contains(&info.location) {
                panic!("Duplicate uniform block location: {}", info.location);
            }

            locations.insert(info.location);
        }
    }
}
