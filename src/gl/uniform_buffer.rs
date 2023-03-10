use std::{marker::PhantomData, rc::Rc};

use crevice::std140::AsStd140;

use crate::{Block, SlView};

use super::{raw, BufferUsage};

/// Stores a uniform block in a buffer on the GPU.
///
/// Instances of `UniformBuffer` can be created with
/// [`Context::create_uniform_buffer`](crate::gl::Context::create_uniform_buffer).
#[derive(Clone)]
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

impl<B: Block<SlView>> UniformBuffer<B> {
    /// # Panics
    ///
    /// Panics if the length of `raw` is not a multiple of the size of
    /// `<U::GlView as AsStd140>::Output`.
    pub(super) fn from_raw(raw: raw::Buffer) -> Self {
        assert!(Self::uniform_size() > 0);
        assert_eq!(raw.len() % Self::uniform_size(), 0);

        Self {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub fn usage(&self) -> BufferUsage {
        self.raw.usage()
    }

    pub fn len(&self) -> usize {
        self.raw.len() / Self::uniform_size()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set(&self, data: B::GlView) {
        self.raw.set(&[data.as_std140()]);
    }

    pub fn binding(&self) -> UniformBufferBinding<B> {
        UniformBufferBinding {
            raw: self.raw.clone(),
            _phantom: PhantomData,
        }
    }

    fn uniform_size() -> usize {
        std::mem::size_of::<<B::GlView as AsStd140>::Output>()
    }
}

impl<B> UniformBufferBinding<B> {
    pub(super) fn raw(&self) -> &raw::Buffer {
        &self.raw
    }
}
