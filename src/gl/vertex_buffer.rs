use std::{marker::PhantomData, rc::Rc};

use crate::{Gl, Sl, ToPod, Vertex};

use super::untyped;

pub struct VertexBuffer<V: Vertex<Sl>> {
    untyped: untyped::Buffer,
    _phantom: PhantomData<V>,
}

#[derive(Clone)]
pub struct VertexBufferBinding<V: Vertex<Sl>> {
    untyped: Rc<untyped::BufferShared>,
    _phantom: PhantomData<V>,
}

impl<V: Vertex<Sl>> VertexBuffer<V> {
    /// # Panics
    ///
    /// Panics if the length of `buffer` is not a multiple of the size of
    /// `V::Pod`.
    pub fn from_untyped(buffer: untyped::Buffer) -> Self {
        assert_eq!(
            buffer.len() % std::mem::size_of::<<V::InGl as ToPod>::Output>(),
            0
        );

        Self {
            untyped: buffer,
            _phantom: PhantomData,
        }
    }

    pub fn set(&self, data: &[V::Pod]) {
        self.untyped.set(data);
    }

    pub fn bind(&self) -> VertexBufferBinding<V> {
        VertexBufferBinding {
            untyped: self.untyped.shared().clone(),
            _phantom: PhantomData,
        }
    }
}
