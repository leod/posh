use std::marker::PhantomData;

use bytemuck::Pod;
use sealed::sealed;

use super::{untyped, ElementType};

pub trait ElementSource {
    #[doc(hidden)]
    fn buffer(&self) -> Option<(untyped::Buffer, ElementType)>;
}

#[sealed]
pub trait Element: Pod {
    const TYPE: ElementType;
}

#[sealed]
impl Element for u16 {
    const TYPE: ElementType = ElementType::U16;
}

#[sealed]
impl Element for u32 {
    const TYPE: ElementType = ElementType::U32;
}

pub struct ElementBuffer<E: Element> {
    pub(crate) untyped: untyped::Buffer,
    _phantom: PhantomData<E>,
}

impl<E: Element> ElementBuffer<E> {
    /// # Panics
    ///
    /// Panics if the length of `untyped` is not a multiple of the size of
    /// `E`.
    pub fn from_untyped(untyped: untyped::Buffer) -> Self {
        assert_eq!(untyped.len() % std::mem::size_of::<E>(), 0);

        Self {
            untyped,
            _phantom: PhantomData,
        }
    }

    pub(crate) fn untyped(&self) -> &untyped::Buffer {
        &self.untyped
    }

    pub fn set(&self, data: &[E]) {
        self.untyped.set(data);
    }
}

impl<E: Element> ElementSource for ElementBuffer<E> {
    fn buffer(&self) -> Option<(untyped::Buffer, ElementType)> {
        Some((self.untyped.clone(), E::TYPE))
    }
}

impl ElementSource for () {
    fn buffer(&self) -> Option<(untyped::Buffer, ElementType)> {
        None
    }
}
