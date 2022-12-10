use std::marker::PhantomData;

use sealed::sealed;

#[sealed]
pub trait Element {}

#[sealed]
impl Element for u16 {}

#[sealed]
impl Element for u32 {}

pub struct ElementBuffer<E: Element> {
    _phantom: PhantomData<E>,
}
