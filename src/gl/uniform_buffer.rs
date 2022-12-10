use std::marker::PhantomData;

use crate::{Gl, Uniform};

pub struct UniformBuffer<U: Uniform<Gl>> {
    _phantom: PhantomData<U>,
}

pub struct UniformBufferBinding<U: Uniform<Gl>> {
    _phantom: PhantomData<U>,
}
