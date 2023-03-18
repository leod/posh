use std::marker::PhantomData;

use crate::{
    interface::FragmentVisitor,
    sl::{self, ColorSample},
    Fragment, Gl,
};

use super::{
    raw::{self},
    ColorSampler2d, Sampler2dParams,
};

pub trait Framebuffer<F: Fragment<Gl>> {
    fn raw(&self) -> raw::Framebuffer;
}

#[derive(Debug, Clone, Default)]
pub struct DefaultFramebuffer {}

#[derive(Clone)]
pub struct ColorAttachment<S> {
    raw: raw::Attachment,
    _phantom: PhantomData<S>,
}

#[derive(Clone)]
pub struct DepthAttachment {
    raw: raw::Attachment,
}

#[derive(Clone)]
pub struct ColorDepthFramebuffer<F: Fragment<Gl>> {
    pub color: F,
    pub depth: DepthAttachment,
}

impl<S> ColorAttachment<S> {
    pub(super) fn from_raw(raw: raw::Attachment) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }

    pub fn color_sampler(&self, params: Sampler2dParams) -> ColorSampler2d<S> {
        ColorSampler2d::from_raw(self.raw.sampler(params, None))
    }
}

impl DepthAttachment {
    pub(super) fn from_raw(raw: raw::Attachment) -> Self {
        Self { raw }
    }
}

impl Framebuffer<ColorAttachment<sl::Vec4>> for DefaultFramebuffer {
    fn raw(&self) -> raw::Framebuffer {
        raw::Framebuffer::Default
    }
}

impl<F: Fragment<Gl>> Framebuffer<F> for F {
    fn raw(&self) -> raw::Framebuffer {
        raw::Framebuffer::Attachments {
            attachments: raw_color_attachments(self),
        }
    }
}

impl<F: Fragment<Gl>> Framebuffer<F> for ColorDepthFramebuffer<F> {
    fn raw(&self) -> raw::Framebuffer {
        let mut attachments = raw_color_attachments(&self.color);
        attachments.push(self.depth.raw.clone());

        raw::Framebuffer::Attachments { attachments }
    }
}

fn raw_color_attachments<F: Fragment<Gl>>(attachments: &F) -> Vec<raw::Attachment> {
    struct Visitor(Vec<raw::Attachment>);

    impl<'a> FragmentVisitor<'a, Gl> for Visitor {
        fn accept<S: ColorSample>(&mut self, _: &str, attachment: &ColorAttachment<S>) {
            self.0.push(attachment.raw.clone());
        }
    }

    // TODO: Remove hardcoded path names.
    let mut visitor = Visitor(Vec::new());
    attachments.visit("fragment_output", &mut visitor);

    visitor.0
}
