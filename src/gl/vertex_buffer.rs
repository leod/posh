use std::{marker::PhantomData, rc::Rc};

use crevice::std140::AsStd140;

use crate::{Block, Sl};

use super::{raw, BufferUsage};

/// Stores vertex data in a buffer on the GPU.
///
/// Instances of `VertexBuffer` can be created with
/// [`Context::create_vertex_buffer`](crate::gl::Context::create_vertex_buffer).
#[derive(Clone)]
pub struct VertexBuffer<V> {
    pub(super) raw: Rc<raw::Buffer>,
    _phantom: PhantomData<V>,
}

impl<V: Block<Sl>> VertexBuffer<V> {
    pub(super) fn from_raw(raw: raw::Buffer) -> Self {
        assert!(vertex_size::<V>() > 0);
        assert_eq!(raw.len() % vertex_size::<V>(), 0);

        Self {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub fn gl(&self) -> &Rc<glow::Context> {
        self.raw.gl()
    }

    pub fn usage(&self) -> BufferUsage {
        self.raw.usage()
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.raw.len() % vertex_size::<V>(), 0);

        self.raw.len() / vertex_size::<V>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set(&self, data: &[V]) {
        todo!()
    }
}

pub(super) const fn vertex_size<V: Block<Sl>>() -> usize {
    std::mem::size_of::<<V::InGl as AsStd140>::Output>()
}
