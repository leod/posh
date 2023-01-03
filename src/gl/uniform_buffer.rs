use std::marker::PhantomData;

use crate::{Sl, Uniform};

#[derive(Clone)]
pub struct UniformBuffer<U: Uniform<Sl>> {
    _phantom: PhantomData<U>,
}

#[derive(Clone)]
pub struct UniformBufferBinding<U: Uniform<Sl>> {
    _phantom: PhantomData<U>,
}
