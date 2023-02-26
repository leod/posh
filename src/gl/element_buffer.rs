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
#[derive(Clone)]
pub struct ElementBuffer<E = u32> {
    raw: Rc<raw::Buffer>,
    _phantom: PhantomData<E>,
}

#[derive(Clone)]
pub struct ElementBufferBinding {
    pub raw: Rc<raw::Buffer>,
    pub ty: ElementType,
    pub element_size: usize,
    pub range: Range<usize>,
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
        self.raw.set(data);
    }

    pub fn binding(&self) -> ElementBufferBinding {
        self.range_binding(0..self.len())
    }

    pub fn range_binding(&self, range: Range<usize>) -> ElementBufferBinding {
        assert!(range.start <= range.end);

        ElementBufferBinding {
            raw: self.raw.clone(),
            ty: E::TYPE,
            element_size: size_of::<E>(),
            range,
        }
    }
}

impl ElementBufferBinding {
    pub(crate) fn raw(&self) -> &raw::Buffer {
        &self.raw
    }

    pub(crate) fn ty(&self) -> ElementType {
        self.ty
    }

    pub fn range(&self) -> Range<usize> {
        self.range.clone()
    }
}
