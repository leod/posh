use std::{collections::BTreeSet, rc::Rc};

use glow::HasContext;

use crate::sl::program_def::ProgramDef;

use super::{
    context::ContextShared, error::check_gl_error, vertex_layout::VertexAttributeLayout, Buffer,
    DrawError, DrawParams, Framebuffer, ProgramError, ProgramValidationError, Sampler, VertexSpec,
};

pub struct Program {
    ctx: Rc<ContextShared>,
    def: ProgramDef,
    id: glow::Program,
}

impl Program {
    pub(super) fn new(ctx: Rc<ContextShared>, def: ProgramDef) -> Result<Self, ProgramError> {
        validate_program_def(&def)?;

        let gl = ctx.gl();

        check_gl_error(gl, "before creating program").map_err(ProgramError::Unexpected)?;

        let id = unsafe { gl.create_program() }.map_err(ProgramError::ProgramCreation)?;
        let program = Self {
            ctx: ctx.clone(),
            def,
            id,
        };

        check_gl_error(gl, "after creating program").map_err(ProgramError::Unexpected)?;

        // Compile and attach shaders.
        let vertex_shader = Shader::new(
            ctx.clone(),
            glow::VERTEX_SHADER,
            &program.def.vertex_shader_source,
        )?
        .attach(program.id);

        check_gl_error(gl, "after compiling vertex shader").map_err(ProgramError::Unexpected)?;

        let fragment_shader = Shader::new(
            ctx.clone(),
            glow::FRAGMENT_SHADER,
            &program.def.fragment_shader_source,
        )?
        .attach(program.id);

        check_gl_error(gl, "after compiling fragment shader").map_err(ProgramError::Unexpected)?;

        // Bind vertex attributes. This needs to be done before linking the
        // program.
        {
            let mut index = 0;

            for block_def in &program.def.vertex_block_defs {
                for attribute in &block_def.attributes {
                    unsafe {
                        gl.bind_attrib_location(
                            program.id,
                            index.try_into().unwrap(),
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

        check_gl_error(gl, "after binding vertex attributes").map_err(ProgramError::Unexpected)?;

        // Link the program.
        let link_status = unsafe {
            gl.link_program(program.id);

            // Note that we do not check shader compile status before checking
            // the program link status, since this would break pipelining and
            // thereby potentially slow down compilation.
            gl.get_program_link_status(program.id)
        };

        check_gl_error(gl, "after linking the program").map_err(ProgramError::Unexpected)?;

        if !link_status {
            let vertex_shader_info = unsafe { gl.get_shader_info_log(vertex_shader.shader.id) };
            let fragment_shader_info = unsafe { gl.get_shader_info_log(fragment_shader.shader.id) };
            let program_info = unsafe { gl.get_program_info_log(program.id) };

            return Err(ProgramError::Compiler {
                vertex_shader_info,
                fragment_shader_info,
                program_info,
            });
        }

        // Set texture units.
        ctx.bind_program(Some(program.id));

        for sampler_def in &program.def.uniform_sampler_defs {
            let location = unsafe { gl.get_uniform_location(program.id, &sampler_def.name) };

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

        check_gl_error(gl, "after setting texture units").map_err(ProgramError::Unexpected)?;

        // Set uniform block locations.
        for uniform_def in &program.def.uniform_block_defs {
            let index = unsafe { gl.get_uniform_block_index(program.id, &uniform_def.block_name) };

            // As with texture units, we silently ignore uniform block index
            // lookup failures here.
            if let Some(index) = index {
                unsafe {
                    gl.uniform_block_binding(
                        program.id,
                        index,
                        u32::try_from(uniform_def.location).unwrap(),
                    );
                }
            }
        }

        check_gl_error(gl, "after setting uniform block locations")
            .map_err(ProgramError::Unexpected)?;

        Ok(program)
    }

    /// # Panics
    ///
    /// Panics under any of the following conditions:
    /// 1. The supplied objects do not belong to the same `glow::Context`.
    /// 2. The wrong number of uniform buffers is supplied.
    /// 3. The wrong number of samplers is supplied.
    /// 4. The vertex stream is not compatible with the program.
    ///
    /// # Safety
    ///
    /// TODO
    pub unsafe fn draw(
        &self,
        uniform_buffers: &[&Buffer],
        samplers: &[Sampler],
        vertex_spec: &VertexSpec,
        framebuffer: &Framebuffer,
        params: &DrawParams,
    ) -> Result<(), DrawError> {
        let ctx = &self.ctx;
        let gl = ctx.gl();
        let def = &self.def;

        assert_eq!(uniform_buffers.len(), def.uniform_block_defs.len());
        assert_eq!(samplers.len(), def.uniform_sampler_defs.len());
        assert!(vertex_spec.is_compatible(&self.def.vertex_block_defs));

        ctx.bind_framebuffer(framebuffer)?;

        let framebuffer_size = framebuffer.size(&self.ctx);

        // `set_draw_params` also clears the screen, so it must come after
        // binding the framebuffer.
        ctx.set_draw_params(params, framebuffer_size);

        ctx.bind_program(Some(self.id));

        for (buffer, block_def) in uniform_buffers.iter().zip(&def.uniform_block_defs) {
            assert!(buffer.context().ref_eq(ctx));

            let location = u32::try_from(block_def.location).unwrap();

            self.ctx.bind_uniform_buffer(location, Some(buffer.id()));
        }

        for (sampler, sampler_def) in samplers.iter().zip(&def.uniform_sampler_defs) {
            assert!(sampler.context().ref_eq(ctx));

            sampler.bind(sampler_def.texture_unit);
        }

        vertex_spec.draw(ctx);

        #[cfg(debug_assertions)]
        check_gl_error(gl, "after draw").map_err(DrawError::Error)?;

        Ok(())
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        self.ctx.unbind_program_if_bound(self.id);

        let gl = self.ctx.gl();

        unsafe {
            gl.delete_program(self.id);
        }
    }
}

struct Shader {
    ctx: Rc<ContextShared>,
    id: glow::Shader,
}

impl Shader {
    fn new(ctx: Rc<ContextShared>, ty: u32, source: &str) -> Result<Self, ProgramError> {
        let gl = ctx.gl();
        let id = unsafe { gl.create_shader(ty) }.map_err(ProgramError::ShaderCreation)?;

        unsafe {
            gl.shader_source(id, source);
            gl.compile_shader(id);
        }

        Ok(Self { ctx, id })
    }

    fn attach(self, program_id: glow::Program) -> AttachedShader {
        let gl = self.ctx.gl();

        unsafe {
            gl.attach_shader(program_id, self.id);
        }

        AttachedShader {
            shader: self,
            program_id,
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        let gl = self.ctx.gl();

        unsafe {
            gl.delete_shader(self.id);
        }
    }
}

struct AttachedShader {
    shader: Shader,
    program_id: glow::Program,
}

impl Drop for AttachedShader {
    fn drop(&mut self) {
        let gl = self.shader.ctx.gl();

        unsafe {
            gl.detach_shader(self.program_id, self.shader.id);
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

    // FIXME: Check that the number of fragment fields is <= MAX_DRAW_BUFFERS.

    Ok(())
}
