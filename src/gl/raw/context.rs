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

pub struct Context {
    gl: Rc<glow::Context>,
    caps: Caps,
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

        Ok(Self {
            gl: Rc::new(gl),
            caps,
        })
    }

    pub fn gl(&self) -> &Rc<glow::Context> {
        &self.gl
    }

    pub fn caps(&self) -> &Caps {
        &self.caps
    }

    pub fn create_buffer<T: Pod>(
        &self,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<Buffer, BufferError> {
        Buffer::new(self.gl.clone(), data, usage)
    }

    pub fn create_texture_2d(&self, image: Image) -> Result<Texture2d, TextureError> {
        Texture2d::new(self.gl.clone(), &self.caps, image)
    }

    pub fn create_texture_2d_with_mipmap(&self, image: Image) -> Result<Texture2d, TextureError> {
        Texture2d::new_with_mipmap(self.gl.clone(), &self.caps, image)
    }

    pub fn create_framebuffer(
        &self,
        attachments: &[FramebufferAttachment],
    ) -> Result<Framebuffer, FramebufferError> {
        Framebuffer::new(self.gl.clone(), &self.caps, attachments)
    }

    pub fn create_program(&self, def: ProgramDef) -> Result<Program, ProgramError> {
        Program::new(self.gl.clone(), def)
    }
}
