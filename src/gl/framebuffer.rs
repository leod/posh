use std::marker::PhantomData;

use crate::{
    interface::FragmentVisitor,
    sl::{self, ColorSample},
    Fragment, Gl,
};

use super::{
    raw::{self},
    Sampler2d, Sampler2dParams,
};

pub trait Framebuffer<F: Fragment<Gl>> {
    fn raw(&self) -> raw::Framebuffer;
}

#[derive(Debug, Clone, Default)]
pub struct DefaultFramebuffer {}

impl Framebuffer<Attachment<sl::Vec4>> for DefaultFramebuffer {
    fn raw(&self) -> raw::Framebuffer {
        raw::Framebuffer::Default
    }
}

impl<F: Fragment<Gl>> Framebuffer<F> for F {
    fn raw(&self) -> raw::Framebuffer {
        raw::Framebuffer::Attachments {
            attachments: raw_attachments(self),
        }
    }
}

#[derive(Clone)]
pub struct Attachment<S> {
    raw: raw::Attachment,
    _phantom: PhantomData<S>,
}

impl<S> Attachment<S> {
    pub(super) fn from_raw(raw: raw::Attachment) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }

    pub fn sampler(&self, params: Sampler2dParams) -> Sampler2d<S> {
        Sampler2d::from_raw(self.raw.sampler(params, None))
    }
}

fn raw_attachments<F: Fragment<Gl>>(attachments: &F) -> Vec<raw::Attachment> {
    struct Visitor(Vec<raw::Attachment>);
    impl<'a> FragmentVisitor<'a, Gl> for Visitor {
        fn accept<S: ColorSample>(&mut self, _: &str, attachment: &Attachment<S>) {
            self.0.push(attachment.raw.clone());
        }
    }

    // TODO: Remove hardcoded path names.
    let mut visitor = Visitor(Vec::new());
    attachments.visit("fragment_output", &mut visitor);

    visitor.0
}
