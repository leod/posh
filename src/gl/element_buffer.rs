use std::{marker::PhantomData, mem::size_of, rc::Rc};

use bytemuck::Pod;
use sealed::sealed;

use super::{raw, ElementType};

pub trait ElementSource {
    #[doc(hidden)]
    fn buffer(&self) -> Option<(&raw::Buffer, ElementType)>;
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

/// Stores element data in a buffer on the GPU.
///
/// Instances of `ElementBuffer` can be created with
/// [`Context::create_element_buffer`](crate::gl::Context::create_element_buffer).
#[derive(Clone)]
pub struct ElementBuffer<E> {
    pub(crate) raw: Rc<raw::Buffer>,
    _phantom: PhantomData<E>,
}

impl<E: Element> ElementBuffer<E> {
    pub(crate) fn from_raw(raw: raw::Buffer) -> Self {
        assert_eq!(raw.len() % size_of::<E>(), 0);

        Self {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.raw.len() % size_of::<E>(), 0);

        self.raw.len() / size_of::<E>()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set(&self, data: &[E]) {
        self.raw.set(data);
    }
}

impl<E: Element> ElementSource for ElementBuffer<E> {
    fn buffer(&self) -> Option<(&raw::Buffer, ElementType)> {
        Some((&self.raw, E::TYPE))
    }
}

impl ElementSource for () {
    fn buffer(&self) -> Option<(&raw::Buffer, ElementType)> {
        None
    }
}
