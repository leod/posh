use std::{cell::Cell, rc::Rc};

use bytemuck::Pod;
use glow::HasContext;

use crate::{
    gl::{BufferError, BufferUsage, ProgramError},
    sl::program_def::ProgramDef,
};

use super::{Buffer, Caps, ContextError, DrawParams, Image, Program, Texture2d, TextureError};

pub(super) struct ContextShared {
    gl: glow::Context,
    caps: Caps,
    draw_params: Cell<DrawParams>,
    draw_fbo: glow::Framebuffer,
    default_framebuffer_size: Cell<glam::UVec2>,
}

pub struct Context {
    shared: Rc<ContextShared>,
}

impl ContextShared {
    pub fn ref_eq(&self, other: &ContextShared) -> bool {
        std::ptr::eq(self as *const ContextShared, other as *const ContextShared)
    }

    pub fn gl(&self) -> &glow::Context {
        &self.gl
    }

    pub fn caps(&self) -> &Caps {
        &self.caps
    }

    pub(super) fn set_draw_params(&self, new: &DrawParams, framebuffer_size: glam::UVec2) {
        let gl = &self.gl;

        let current = self.draw_params.get();
        new.set_delta(gl, &current, framebuffer_size);
        self.draw_params.set(*new);
    }

    pub(super) fn draw_fbo(&self) -> glow::Framebuffer {
        self.draw_fbo
    }

    pub(super) fn default_framebuffer_size(&self) -> glam::UVec2 {
        self.default_framebuffer_size.get()
    }
}

impl Context {
    pub fn new(gl: glow::Context) -> Result<Self, ContextError> {
        let caps = Caps::new(&gl);

        // All vertex bindings are made through a single vertex array object
        // that is bound at the start. The vertex array object binding must not
        // be changed during the lifetime of a context.
        let vao = unsafe { gl.create_vertex_array() }.map_err(ContextError::ObjectCreation)?;

        unsafe { gl.bind_vertex_array(Some(vao)) };

        // All framebuffer attachments are made with a single framebuffer object
        // that is created at the start.
        let draw_fbo = unsafe { gl.create_framebuffer() }.map_err(ContextError::ObjectCreation)?;

        let default_framebuffer_size = {
            let mut viewport = [0, 0, 0, 0];

            unsafe { gl.get_parameter_i32_slice(glow::VIEWPORT, &mut viewport) };

            glam::uvec2(
                viewport[2].try_into().unwrap(),
                viewport[3].try_into().unwrap(),
            )
        };

        let shared = Rc::new(ContextShared {
            gl,
            caps,
            draw_params: Cell::new(DrawParams::default()),
            draw_fbo,
            default_framebuffer_size: Cell::new(default_framebuffer_size),
        });

        Ok(Self { shared })
    }

    pub fn caps(&self) -> &Caps {
        &self.shared.caps
    }

    pub fn create_buffer<T: Pod>(
        &self,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<Buffer, BufferError> {
        Buffer::new(self.shared.clone(), data, usage)
    }

    pub fn create_texture_2d(&self, image: Image) -> Result<Texture2d, TextureError> {
        Texture2d::new(self.shared.clone(), image)
    }

    pub fn create_texture_2d_with_mipmap(&self, image: Image) -> Result<Texture2d, TextureError> {
        Texture2d::new_with_mipmap(self.shared.clone(), image)
    }

    pub fn create_program(&self, def: ProgramDef) -> Result<Program, ProgramError> {
        Program::new(self.shared.clone(), def)
    }

    pub(crate) fn default_framebuffer_size(&self) -> glam::UVec2 {
        self.shared.default_framebuffer_size.get()
    }

    pub(crate) fn set_default_framebuffer_size(&self, size: glam::UVec2) {
        self.shared.default_framebuffer_size.set(size);
    }
}
