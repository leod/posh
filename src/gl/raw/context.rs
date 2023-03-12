use std::rc::Rc;

use bytemuck::Pod;
use glow::HasContext;

use crate::{
    gl::{BufferError, BufferUsage, ProgramError},
    sl::program_def::ProgramDef,
};

use super::{
    Buffer, Caps, ContextError, Framebuffer, FramebufferAttachment, FramebufferError, Image,
    Program, Texture2d, TextureError,
};

pub(super) struct ContextShared {
    gl: glow::Context,
    caps: Caps,
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
}

impl Context {
    pub fn new(gl: glow::Context) -> Result<Self, ContextError> {
        let caps = Caps::new(&gl);

        // All vertex bindings are made through a single vertex array object
        // that is bound at the start. The vertex array object binding must not
        // be changed during the lifetime of a context.
        let vao = unsafe { gl.create_vertex_array() }.map_err(ContextError::ObjectCreation)?;

        unsafe {
            gl.bind_vertex_array(Some(vao));
        }

        let shared = Rc::new(ContextShared { gl, caps });

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

    pub fn create_framebuffer(
        &self,
        attachments: &[FramebufferAttachment],
    ) -> Result<Framebuffer, FramebufferError> {
        Framebuffer::new(self.shared.clone(), attachments)
    }

    pub fn create_program(&self, def: ProgramDef) -> Result<Program, ProgramError> {
        Program::new(self.shared.clone(), def)
    }

    pub fn clear_color(&self, color: glam::Vec4) {
        let gl = self.shared.gl();

        unsafe {
            gl.clear_color(color.x, color.y, color.z, color.w);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn clear_color_and_depth(&self, color: glam::Vec4, depth: f32) {
        let gl = self.shared.gl();

        unsafe {
            gl.clear_color(color.x, color.y, color.z, color.w);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }
}
