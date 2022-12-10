use std::marker::PhantomData;

use crate::Numeric;

pub struct Sampler2d<T: Numeric> {
    _phantom: PhantomData<T>,
}

pub struct Sampler2dBinding<T: Numeric> {
    _phantom: PhantomData<T>,
}
