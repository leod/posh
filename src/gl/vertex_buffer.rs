use std::marker::PhantomData;

use crate::{Sl, ToPod, Vertex};

use super::untyped;

#[derive(Clone)]
pub struct VertexBuffer<V: Vertex<Sl>> {
    pub(crate) untyped: untyped::Buffer,
    _phantom: PhantomData<V>,
}

impl<V: Vertex<Sl>> VertexBuffer<V> {
    /// # Panics
    ///
    /// Panics if the length of `untyped` is not a multiple of the size of
    /// `V::Pod`.
    pub fn from_untyped(untyped: untyped::Buffer) -> Self {
        assert_eq!(
            untyped.len() % std::mem::size_of::<<V::InGl as ToPod>::Output>(),
            0
        );

        Self {
            untyped,
            _phantom: PhantomData,
        }
    }

    pub fn set(&self, data: &[V::Pod]) {
        self.untyped.set(data);
    }
}
