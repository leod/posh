use std::{marker::PhantomData, mem::size_of};

use bytemuck::Pod;
use sealed::sealed;

use super::{untyped, ElementType};

pub trait ElementSource {
    #[doc(hidden)]
    fn buffer(&self) -> Option<(untyped::Buffer, ElementType)>;
}

#[sealed]
pub trait ElementOrUnit {
    type Source: ElementSource;
}

#[sealed]
impl ElementOrUnit for u16 {
    type Source = ElementBuffer<Self>;
}

#[sealed]
impl ElementOrUnit for u32 {
    type Source = ElementBuffer<Self>;
}

#[sealed]
impl ElementOrUnit for () {
    type Source = ();
}

#[sealed]
pub trait Element: Pod + ElementOrUnit<Source = ElementBuffer<Self>> {
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

#[derive(Clone)]
pub struct ElementBuffer<E> {
    pub(crate) untyped: untyped::Buffer,
    _phantom: PhantomData<E>,
}

impl<E: Element> ElementBuffer<E> {
    /// # Panics
    ///
    /// Panics if the length of `untyped` is not a multiple of the size of
    /// `E`.
    pub(crate) fn from_untyped(untyped: untyped::Buffer) -> Self {
        assert_eq!(untyped.len() % size_of::<E>(), 0);

        Self {
            untyped,
            _phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.untyped.len() / size_of::<E>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
