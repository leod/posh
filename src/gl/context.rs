use std::rc::Rc;

use bytemuck::Pod;

use crate::{Sl, Vertex};

use super::{untyped, BufferUsage, CreateBufferError, VertexBuffer};

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

    pub fn untyped_create_buffer<T: Pod>(
        &self,
        data: &[T],
        usage: BufferUsage,
    ) -> Result<untyped::Buffer, CreateBufferError> {
        untyped::Buffer::new(self.gl.clone(), data, usage)
    }
}
