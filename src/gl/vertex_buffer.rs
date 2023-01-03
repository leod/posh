use std::{marker::PhantomData, rc::Rc};

use crate::{Sl, ToPod, Vertex};

use super::{untyped, BufferUsage};

#[derive(Clone)]
pub struct VertexBuffer<V> {
    pub(crate) untyped: untyped::Buffer,
    _phantom: PhantomData<V>,
}

impl<V: Vertex<Sl>> VertexBuffer<V> {
    /// # Panics
    ///
    /// Panics if the length of `untyped` is not a multiple of the size of
    /// `V::Pod`.
    ///
    /// # TODO
    ///
    /// Since `untyped::Buffer` is `Rc`-cloneable, the underlying buffer can
    /// still be modified. Check if we want to allow this.
    pub(crate) fn from_untyped(untyped: untyped::Buffer) -> Self {
        assert!(Self::vertex_size() > 0);
        assert_eq!(untyped.len() % Self::vertex_size(), 0);

        Self {
            untyped,
            _phantom: PhantomData,
        }
    }

    pub fn gl(&self) -> &Rc<glow::Context> {
        self.untyped.gl()
    }

    pub fn usage(&self) -> BufferUsage {
        self.untyped.usage()
    }

    pub fn len(&self) -> usize {
        self.untyped.len() / Self::vertex_size()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set(&self, data: &[V::Pod]) {
        self.untyped.set(data);
    }

    fn vertex_size() -> usize {
        std::mem::size_of::<<V::InGl as ToPod>::Output>()
    }
}
