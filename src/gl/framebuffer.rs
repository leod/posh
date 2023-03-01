use std::marker::PhantomData;

use crate::{sl::Sample, Fragment, SlView};

pub struct Framebuffer<F> {
    _phantom: PhantomData<F>,
}

impl<F: Fragment<SlView>> Framebuffer<F> {}

pub struct FramebufferAttachment<S: Sample> {
    _phantom: PhantomData<S>,
}

pub struct FramebufferBinding<F> {
    _phantom: PhantomData<F>,
}

impl Default for FramebufferBinding<glam::Vec4> {
    fn default() -> Self {
        todo!()
    }
}
