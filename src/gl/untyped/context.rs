use std::rc::Rc;

use bytemuck::Pod;

use crate::gl::{BufferUsage, CreateBufferError, CreateVertexDataError};

use super::{Buffer, VertexBinding, VertexBindingBufferInfo};

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

    pub fn create_vertex_binding(
        &self,
        vertex_buffers: &[(Buffer, VertexBindingBufferInfo)],
    ) -> Result<VertexBinding, CreateVertexDataError> {
        VertexBinding::new(self.gl.clone(), vertex_buffers)
    }
}
