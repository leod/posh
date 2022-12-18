use std::marker::PhantomData;

use crate::{Gl, Sl, Uniform};

pub trait AsStd140 {
    type AsStd140: crevice::std140::AsStd140;
}

pub struct UniformBuffer<U: Uniform<Sl>> {
    _phantom: PhantomData<U>,
}

#[derive(Clone)]
pub struct UniformBufferBinding<U: Uniform<Sl>> {
    _phantom: PhantomData<U>,
}
