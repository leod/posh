use std::rc::Rc;

use bytemuck::Pod;

use crate::gl::{BufferUsage, CreateBufferError, CreateVertexStreamError, ElementType};

use super::{Buffer, VertexStream, VertexStreamVertexInfo};

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

    pub fn create_vertex_stream(
        &self,
        vertex_buffers: &[(Buffer, VertexStreamVertexInfo)],
        element_buffer: Option<(Buffer, ElementType)>,
    ) -> Result<VertexStream, CreateVertexStreamError> {
        VertexStream::new(self.gl.clone(), vertex_buffers, element_buffer)
    }
}
