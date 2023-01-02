use std::rc::Rc;

use bytemuck::Pod;

use crate::{Sl, Vertex, VertexInterface};

use super::{
    untyped, BufferUsage, CreateBufferError, CreateVertexDataError, VertexBuffer, VertexData,
};

pub(crate) struct ContextShared {}

pub struct Context {
    shared: Rc<ContextShared>,
    gl: Rc<glow::Context>,
}

impl Context {
    pub(crate) fn shared(&self) -> &Rc<ContextShared> {
        &self.shared
    }

    pub fn gl(&self) -> &Rc<glow::Context> {
        &self.gl
    }

    pub fn create_buffer<V: Vertex<Sl>>(
        &self,
        data: &[V::Pod],
        usage: BufferUsage,
    ) -> Result<VertexBuffer<V>, CreateBufferError> {
        let buffer = self.untyped_create_buffer(data, usage)?;

        Ok(VertexBuffer::from_untyped(buffer))
    }

    pub fn create_vertex_data<V: VertexInterface<Sl>>(
        &self,
        bindings: V::InGl,
    ) -> Result<VertexData<V>, CreateVertexDataError> {
        VertexData::new(self, bindings)
    }

    pub fn untyped_create_buffer<T: Pod>(
        &self,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<untyped::Buffer, CreateBufferError> {
        untyped::Buffer::new(self.gl.clone(), data, usage)
    }

    pub fn untyped_create_vertex_data(
        &self,
        bindings_and_entry_infos: &[(untyped::BufferBinding, untyped::VertexDataEntryInfo)],
    ) -> Result<untyped::VertexData, CreateVertexDataError> {
        untyped::VertexData::new(self.gl.clone(), bindings_and_entry_infos)
    }
}
