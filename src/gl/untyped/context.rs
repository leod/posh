use std::rc::Rc;

use bytemuck::Pod;

use crate::gl::{BufferUsage, CreateBufferError, CreateVertexDataError};

use super::{Buffer, BufferBinding, VertexData, VertexDataEntryInfo};

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

    pub fn create_vertex_data(
        &self,
        vertex_bindings_and_entry_infos: &[(BufferBinding, VertexDataEntryInfo)],
    ) -> Result<VertexData, CreateVertexDataError> {
        VertexData::new(self.gl.clone(), vertex_bindings_and_entry_infos)
    }
}
