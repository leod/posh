use std::rc::Rc;

use crate::{Sl, Vertex, VertexInterface};

use super::{
    untyped, BufferUsage, CreateBufferError, CreateVertexDataError, VertexBuffer, VertexData,
};

pub struct Context {
    gl: Rc<glow::Context>,
    untyped: untyped::Context,
}

impl Context {
    pub fn gl(&self) -> &Rc<glow::Context> {
        &self.gl
    }

    pub fn create_buffer<V: Vertex<Sl>>(
        &self,
        data: &[V::Pod],
        usage: BufferUsage,
    ) -> Result<VertexBuffer<V>, CreateBufferError> {
        let buffer = self.untyped.create_buffer(data, usage)?;

        Ok(VertexBuffer::from_untyped(buffer))
    }

    pub fn create_vertex_data<V: VertexInterface<Sl>>(
        &self,
        bindings: V::InGl,
    ) -> Result<VertexData<V>, CreateVertexDataError> {
        VertexData::new(self, bindings)
    }

    pub fn untyped(&self) -> &untyped::Context {
        &self.untyped
    }
}
