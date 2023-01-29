use std::marker::PhantomData;

use crate::sl::Sample;

pub struct Sampler2d<S: Sample> {
    _phantom: PhantomData<S>,
}
