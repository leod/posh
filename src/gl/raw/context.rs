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

    pub(super) fn set_draw_params(&self, new: &DrawParams) {
        let gl = &self.gl;

        let current = self.draw_params.get();
        new.set_delta(gl, &current);
        self.draw_params.set(*new);
    }

    pub(super) fn draw_fbo(&self) -> glow::Framebuffer {
        self.draw_fbo
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

        let shared = Rc::new(ContextShared {
            gl,
            caps,
            draw_params: Cell::new(DrawParams::default()),
            draw_fbo,
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

    pub fn clear_color(&self, color: glam::Vec4) {
        let gl = self.shared.gl();

        unsafe { gl.clear_color(color.x, color.y, color.z, color.w) };
        unsafe { gl.clear(glow::COLOR_BUFFER_BIT) };
    }

    pub fn clear_color_and_depth(&self, color: glam::Vec4, depth: f32) {
        let gl = self.shared.gl();

        unsafe { gl.clear_color(color.x, color.y, color.z, color.w) };
        unsafe { gl.clear_depth_f32(depth) };
        unsafe { gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT) };
    }
}
