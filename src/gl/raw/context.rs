use std::rc::Rc;

use bytemuck::Pod;

use crate::{
    gl::{BufferError, BufferUsage, ElementType, ProgramError, VertexArrayError},
    sl::program_def::{ProgramDef, VertexDef},
};

use super::{Buffer, Caps, Image, Program, Texture2d, TextureError, VertexArray};

pub struct Context {
    gl: Rc<glow::Context>,
    caps: Caps,
}

impl Context {
    pub fn new(gl: glow::Context) -> Self {
        let caps = Caps::new(&gl);

        Self {
            gl: Rc::new(gl),
            caps,
        }
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

    pub fn create_vertex_array(
        &self,
        vertex_buffers: &[(&Buffer, VertexDef)],
        element_buffer: Option<(&Buffer, ElementType)>,
    ) -> Result<VertexArray, VertexArrayError> {
        VertexArray::new(self.gl.clone(), vertex_buffers, element_buffer)
    }

    pub fn create_program(&self, def: ProgramDef) -> Result<Program, ProgramError> {
        Program::new(self.gl.clone(), def)
    }

    pub fn create_texture_2d(&self, image: Image) -> Result<Texture2d, TextureError> {
        Texture2d::new(self.gl.clone(), &self.caps, image)
    }

    pub fn create_texture_2d_with_mipmap(&self, image: Image) -> Result<Texture2d, TextureError> {
        Texture2d::new_with_mipmap(self.gl.clone(), &self.caps, image)
    }
}
