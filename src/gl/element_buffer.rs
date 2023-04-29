use std::{marker::PhantomData, mem::size_of, ops::Range, rc::Rc};

use bytemuck::Pod;
use sealed::sealed;

use super::{raw, ElementType};

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

/// Stores element data in a buffer on the GPU.
///
/// Instances of `ElementBuffer` can be created with
/// [`Context::create_element_buffer`](crate::gl::Context::create_element_buffer).
pub struct ElementBuffer<E = u32> {
    raw: Rc<raw::Buffer>,
    _phantom: PhantomData<E>,
}

#[derive(Clone)]
pub struct ElementBufferBinding {
    raw: Rc<raw::Buffer>,
    ty: ElementType,
    range: Range<usize>,
}

impl<E: Element> ElementBuffer<E> {
    pub(super) fn from_raw(raw: raw::Buffer) -> Self {
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
        self.raw.set(bytemuck::cast_slice(data));
    }

    pub fn as_binding(&self) -> ElementBufferBinding {
        ElementBufferBinding {
            raw: self.raw.clone(),
            ty: E::TYPE,
            range: 0..self.len(),
        }
    }
}

impl ElementBufferBinding {
    pub(crate) fn raw(&self) -> &Rc<raw::Buffer> {
        &self.raw
    }

    pub(crate) fn ty(&self) -> ElementType {
        self.ty
    }

    pub(crate) fn range(&self) -> Range<usize> {
        self.range.clone()
    }

    pub fn with_range(mut self, range: Range<usize>) -> Self {
        self.range = range;
        self
    }
}
