use std::rc::Rc;

use crate::{Sl, ToPod, Vertex, VertexInterface};

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

    pub fn create_vertex_buffer<V: Vertex<Sl>>(
        &self,
        data: &[V::InGl],
        usage: BufferUsage,
    ) -> Result<VertexBuffer<V>, CreateBufferError> {
        // TODO: This extra allocation for converting to `V::Pod` could be
        // eliminated if we see the need.
        let data: Vec<_> = data.iter().copied().map(ToPod::to_pod).collect();

        // TODO: We should also allow passing `V::Pod` directly.

        let buffer = self.untyped.create_buffer(&data, usage)?;

        Ok(VertexBuffer::from_untyped(buffer))
    }

    pub fn create_vertex_data<V: VertexInterface<Sl>>(
        &self,
        buffers: V::InGl,
    ) -> Result<VertexData<V>, CreateVertexDataError> {
        VertexData::new(self, buffers)
    }

    pub fn untyped(&self) -> &untyped::Context {
        &self.untyped
    }
}
