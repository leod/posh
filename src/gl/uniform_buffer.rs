use std::{marker::PhantomData, rc::Rc};

use crevice::std140::AsStd140;

use crate::{Sl, Uniform};

use super::{untyped, BufferUsage};

#[derive(Clone)]
pub struct UniformBuffer<U> {
    pub(crate) untyped: untyped::Buffer,
    _phantom: PhantomData<U>,
}

#[derive(Clone)]
pub struct UniformBufferBinding<U> {
    pub(crate) untyped: untyped::Buffer,
    _phantom: PhantomData<U>,
    // TODO: Uniform buffer slicing.
}

impl<U: Uniform<Sl>> UniformBuffer<U> {
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
        assert!(Self::uniform_size() > 0);
        assert_eq!(untyped.len() % Self::uniform_size(), 0);

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
