use std::{marker::PhantomData, rc::Rc};

use crevice::std140::AsStd140;

use crate::{Block, Sl};

use super::{raw, BufferUsage};

/// Stores vertex blocks in a buffer on the GPU.
///
/// Instances of `VertexBuffer` can be created with
/// [`Context::create_vertex_buffer`](crate::gl::Context::create_vertex_buffer).
pub struct VertexBuffer<B> {
    raw: Rc<raw::Buffer>,
    _phantom: PhantomData<B>,
}

#[derive(Clone)]
pub struct VertexBufferBinding<B> {
    raw: Rc<raw::Buffer>,
    _phantom: PhantomData<B>,
    // TODO: Uniform buffer slicing.
}

impl<B: Block<Sl>> VertexBuffer<B> {
    pub(super) fn from_raw(raw: raw::Buffer) -> Self {
        assert!(vertex_size::<B>() > 0);
        assert_eq!(raw.len() % vertex_size::<B>(), 0);

        Self {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub fn usage(&self) -> BufferUsage {
        self.raw.usage()
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.raw.len() % vertex_size::<B>(), 0);

        self.raw.len() / vertex_size::<B>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set(&self, data: &[B]) {
        todo!()
    }

    pub fn binding(&self) -> VertexBufferBinding<B> {
        VertexBufferBinding {
            raw: self.raw.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<B: Block<Sl>> VertexBufferBinding<B> {
    pub(crate) fn raw(&self) -> &Rc<raw::Buffer> {
        &self.raw
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.raw.len() % vertex_size::<B>(), 0);

        self.raw.len() / vertex_size::<B>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

fn vertex_size<V: Block<Sl>>() -> usize {
    std::mem::size_of::<<V::Gl as AsStd140>::Output>()
}
