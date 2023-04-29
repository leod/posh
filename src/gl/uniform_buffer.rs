use std::{marker::PhantomData, rc::Rc};

use crevice::std140::{AsStd140, Std140};

use crate::{Block, Sl};

use super::{raw, BufferUsage};

/// Stores a uniform block in a buffer on the GPU.
///
/// Instances of `UniformBuffer` can be created with
/// [`Context::create_uniform_buffer`](crate::gl::Context::create_uniform_buffer).
pub struct UniformBuffer<B> {
    pub(super) raw: Rc<raw::Buffer>,
    _phantom: PhantomData<B>,
}

#[derive(Clone)]
pub struct UniformBufferBinding<B> {
    raw: Rc<raw::Buffer>,
    _phantom: PhantomData<B>,
    // TODO: Uniform buffer slicing.
}

impl<B: Block<Sl>> UniformBuffer<B> {
    /// # Panics
    ///
    /// Panics if the length of `raw` is not a multiple of the size of
    /// `<U::Gl as AsStd140>::Output`.
    pub(super) fn from_raw(raw: raw::Buffer) -> Self {
        assert!(uniform_size::<B>() > 0);
        assert_eq!(raw.len() % uniform_size::<B>(), 0);
        assert_eq!(raw.len() / uniform_size::<B>(), 1);

        Self {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub fn usage(&self) -> BufferUsage {
        self.raw.usage()
    }

    pub fn set(&self, data: B::Gl) {
        self.raw.set(data.as_std140().as_bytes());

        assert_eq!(self.raw.len() % uniform_size::<B>(), 0);
    }

    pub fn as_binding(&self) -> UniformBufferBinding<B> {
        UniformBufferBinding {
            raw: self.raw.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<B> UniformBufferBinding<B> {
    pub(super) fn raw(&self) -> &raw::Buffer {
        &self.raw
    }
}

fn uniform_size<B: Block<Sl>>() -> usize {
    std::mem::size_of::<<B::Gl as AsStd140>::Output>()
}
