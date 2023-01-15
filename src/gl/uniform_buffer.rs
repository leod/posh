use std::{marker::PhantomData, rc::Rc};

use crevice::std140::AsStd140;

use crate::{Block, Sl};

use super::{untyped, BufferUsage};

/// Stores uniform data in a buffer on the GPU.
///
/// Instances of `UniformBuffer` can be created with
/// [`Context::create_uniform_buffer`](crate::gl::Context::create_uniform_buffer).
#[derive(Clone)]
pub struct UniformBuffer<U> {
    pub(crate) untyped: Rc<untyped::Buffer>,
    _phantom: PhantomData<U>,
}

#[derive(Clone)]
pub struct UniformBufferBinding<U> {
    pub(crate) untyped: Rc<untyped::Buffer>,
    _phantom: PhantomData<U>,
    // TODO: Uniform buffer slicing.
}

impl<U: Block<Sl>> UniformBuffer<U> {
    /// # Panics
    ///
    /// Panics if the length of `untyped` is not a multiple of the size of
    /// `<U::InGl as AsStd140>::Output`.
    pub(crate) fn from_untyped(untyped: untyped::Buffer) -> Self {
        assert!(Self::uniform_size() > 0);
        assert_eq!(untyped.len() % Self::uniform_size(), 0);

        Self {
            untyped: Rc::new(untyped),
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
        self.untyped.len() / Self::uniform_size()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set(&self, data: U::InGl) {
        self.untyped.set(&[data.as_std140()]);
    }

    pub fn bind(&self) -> UniformBufferBinding<U> {
        UniformBufferBinding {
            untyped: self.untyped.clone(),
            _phantom: PhantomData,
        }
    }

    fn uniform_size() -> usize {
        std::mem::size_of::<<U::InGl as AsStd140>::Output>()
    }
}
