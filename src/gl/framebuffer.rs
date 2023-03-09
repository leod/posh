use std::marker::PhantomData;

use crate::{
    interface::FragmentVisitor,
    sl::{self, Sample},
    Fragment, GlView, SlView,
};

use super::raw::{self, FramebufferError};

pub struct FramebufferAttachment<S> {
    _phantom: PhantomData<S>,
}

pub struct Framebuffer<F> {
    _phantom: PhantomData<F>,
}

impl<F: Fragment<SlView>> Framebuffer<F> {}

pub struct FramebufferBinding<F> {
    _phantom: PhantomData<F>,
}

impl<F: Fragment<SlView>> Framebuffer<F> {
    pub(super) fn new(
        context: &raw::Context,
        attachments: F::GlView,
    ) -> Result<Self, FramebufferError> {
    }
}

impl Default for FramebufferBinding<FramebufferAttachment<sl::Vec4>> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

fn raw_attachments<F: Fragment<SlView>>(attachments: &F) -> Vec<raw::FramebufferAttachment> {
    struct Visitor<'a>(Vec<raw::FramebufferAttachment<'a>>);

    impl<'a> FragmentVisitor<GlView> for Visitor<'a> {
        fn accept<S: Sample>(&mut self, path: &str, attachment: &FramebufferAttachment<S>) {
            todo!()
        }
    }
}
