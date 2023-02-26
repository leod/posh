use std::{collections::BTreeSet, rc::Rc};

use glow::HasContext;

use crate::{
    gl::ProgramError,
    sl::program_def::{ProgramDef, UniformSamplerDef},
};

use super::{
    error::check_gl_error, vertex_layout::VertexAttributeLayout, Buffer, ProgramValidationError,
    Sampler, VertexArrayBinding,
};

struct ProgramShared {
    gl: Rc<glow::Context>,
    def: ProgramDef,
    id: glow::Program,
}

pub struct Program {
    shared: Rc<ProgramShared>,
}

impl Program {
    pub(super) fn new(gl: Rc<glow::Context>, def: ProgramDef) -> Result<Self, ProgramError> {
        validate_program_def(&def)?;

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

            for block_def in &shared.def.vertex_block_defs {
                for attribute in &block_def.attributes {
                    unsafe {
                        gl.bind_attrib_location(
                            shared.id,
                            u32::try_from(index).unwrap(),
                            &attribute.name,
                        );
                    }

                    let attribute_info = VertexAttributeLayout::new(attribute.ty)
                        .map_err(ProgramError::InvalidVertexAttribute)?;

                    // Some attributes (e.g. matrices) take up multiple
                    // locations. We only need to bind to the first location,
                    // though.
                    index += attribute_info.locations;
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
        unsafe {
            gl.use_program(Some(shared.id));
        }

        for sampler_def in &shared.def.uniform_sampler_defs {
            let location = unsafe { gl.get_uniform_location(shared.id, &sampler_def.name) };

            // We silently ignore location lookup failures here, since program
            // linking is allowed to remove uniforms that are not used by the
            // program.
            unsafe {
                gl.uniform_1_i32(
                    location.as_ref(),
                    i32::try_from(sampler_def.texture_unit).unwrap(),
                );
            }
        }

        unsafe {
            gl.use_program(None);
        }

        // Set uniform block locations.
        for uniform_def in &shared.def.uniform_block_defs {
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

        check_gl_error(&gl).map_err(ProgramError::Unexpected)?;

        Ok(Program { shared })
    }

    /// # Panics
    ///
    /// Panics if any of the supplied objects do not belong to the same
    /// `glow::Context`, or if the wrong number of uniform buffers is supplied,
    /// or if the wrong number of samplers is specified.
    ///
    /// TOOD: Check vertex array compatibility?
    ///
    /// # Safety
    ///
    /// TODO
    pub unsafe fn draw(
        &self,
        uniform_buffers: &[&Buffer],
        samplers: &[Sampler],
        vertices: VertexArrayBinding,
    ) {
        let gl = &self.shared.gl;
        let def = &self.shared.def;

        assert_eq!(uniform_buffers.len(), def.uniform_block_defs.len());
        assert_eq!(samplers.len(), def.uniform_sampler_defs.len());
        assert!(Rc::ptr_eq(vertices.gl(), gl));

        unsafe {
            gl.use_program(Some(self.shared.id));
        }

        for (buffer, block_def) in uniform_buffers.iter().zip(&def.uniform_block_defs) {
            assert!(Rc::ptr_eq(buffer.gl(), gl));

            let location = u32::try_from(block_def.location).unwrap();

            unsafe {
                gl.bind_buffer_base(glow::UNIFORM_BUFFER, location, Some(buffer.id()));
            }
        }

        for (sampler, sampler_def) in samplers.iter().zip(&def.uniform_sampler_defs) {
            assert!(Rc::ptr_eq(sampler.gl(), gl));

            let unit = texture_unit_gl(sampler_def);

            unsafe {
                gl.active_texture(unit);
            }

            sampler.bind();
        }

        vertices.draw();

        // TODO: Remove overly conservative unbinding.
        for (sampler, sampler_def) in samplers.iter().zip(&def.uniform_sampler_defs) {
            let unit = texture_unit_gl(sampler_def);

            unsafe {
                gl.active_texture(unit);
            }

            sampler.unbind();
        }

        // TODO: Remove overly conservative unbinding.
        for block_def in &def.uniform_block_defs {
            let location = u32::try_from(block_def.location).unwrap();

            unsafe {
                gl.bind_buffer_base(glow::UNIFORM_BUFFER, location, None);
            }
        }

        gl.bind_buffer(glow::UNIFORM_BUFFER, None);

        unsafe {
            gl.use_program(None);
        }

        check_gl_error(gl).unwrap();
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

fn validate_program_def(def: &ProgramDef) -> Result<(), ProgramValidationError> {
    {
        let mut names: BTreeSet<_> = BTreeSet::new();

        for info in &def.uniform_sampler_defs {
            if !names.insert(info.name.clone()) {
                return Err(ProgramValidationError::DuplicateSampler(info.name.clone()));
            }
        }
    }

    {
        let mut texture_units: BTreeSet<_> = BTreeSet::new();

        for sampler_def in &def.uniform_sampler_defs {
            if !texture_units.insert(sampler_def.texture_unit) {
                return Err(ProgramValidationError::DuplicateSamplerTextureUnit(
                    sampler_def.texture_unit,
                ));
            }
        }
    }

    {
        let mut names: BTreeSet<_> = BTreeSet::new();

        for info in &def.uniform_block_defs {
            if !names.insert(info.block_name.clone()) {
                return Err(ProgramValidationError::DuplicateUniformBlock(
                    info.block_name.clone(),
                ));
            }
        }
    }

    {
        let mut locations: BTreeSet<_> = BTreeSet::new();

        for info in &def.uniform_block_defs {
            if !locations.insert(info.location) {
                return Err(ProgramValidationError::DuplicateUniformBlockLocation(
                    info.location,
                ));
            }
        }
    }

    Ok(())
}

fn texture_unit_gl(sampler_def: &UniformSamplerDef) -> u32 {
    u32::try_from(sampler_def.texture_unit)
        .unwrap()
        .checked_add(glow::TEXTURE0)
        .unwrap()
}
