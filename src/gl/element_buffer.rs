use std::{marker::PhantomData, mem::size_of, rc::Rc};

use bytemuck::Pod;
use sealed::sealed;

use super::{untyped, ElementType};

pub trait ElementSource {
    #[doc(hidden)]
    fn buffer(&self) -> Option<(&untyped::Buffer, ElementType)>;
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
/// Instances of `ElementBuffer` can be created with [`Context::create_element_buffer`](crate::gl::Context::create_element_buffer).
#[derive(Clone)]
pub struct ElementBuffer<E> {
    pub(crate) untyped: Rc<untyped::Buffer>,
    _phantom: PhantomData<E>,
}

impl<E: Element> ElementBuffer<E> {
    pub(crate) fn from_untyped(untyped: untyped::Buffer) -> Self {
        assert_eq!(untyped.len() % size_of::<E>(), 0);

        Self {
            untyped: Rc::new(untyped),
            _phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.untyped.len() % size_of::<E>(), 0);

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
    fn buffer(&self) -> Option<(&untyped::Buffer, ElementType)> {
        Some((&self.untyped, E::TYPE))
    }
}

impl ElementSource for () {
    fn buffer(&self) -> Option<(&untyped::Buffer, ElementType)> {
        None
    }
}
