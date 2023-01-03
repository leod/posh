use std::rc::Rc;

use bytemuck::Pod;

use crate::gl::{BufferUsage, CreateBufferError, CreateVertexArrayError, ElementType};

use super::{Buffer, VertexArray, VertexInfo};

pub struct Context {
    gl: Rc<glow::Context>,
}

impl Context {
    pub fn gl(&self) -> &Rc<glow::Context> {
        &self.gl
    }

    pub fn create_buffer<T: Pod>(
        &self,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<Buffer, CreateBufferError> {
        Buffer::new(self.gl.clone(), data, usage)
    }

    pub fn create_vertex_array(
        &self,
        vertex_buffers: &[(Buffer, VertexInfo)],
        element_buffer: Option<(Buffer, ElementType)>,
    ) -> Result<VertexArray, CreateVertexArrayError> {
        VertexArray::new(self.gl.clone(), vertex_buffers, element_buffer)
    }
}
