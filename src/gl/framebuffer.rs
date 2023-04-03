use std::marker::PhantomData;

use crate::{
    interface::FragmentVisitor,
    sl::{self, ColorSample},
    Fragment, Gl, Sl,
};

use super::{
    raw::{self},
    ColorSampler2d, Sampler2dSettings,
};

#[derive(Clone)]
pub struct ColorAttachment<S> {
    raw: raw::Attachment,
    _phantom: PhantomData<S>,
}

impl<S> ColorAttachment<S> {
    pub(super) fn from_raw(raw: raw::Attachment) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }

    pub fn as_color_sampler(&self, settings: Sampler2dSettings) -> ColorSampler2d<S> {
        ColorSampler2d::from_raw(self.raw.sampler(settings, None))
    }
}

impl<'a, S: ColorSample> Into<Framebuffer<S>> for &'a ColorAttachment<S> {
    fn into(self) -> Framebuffer<S> {
        Framebuffer(FramebufferInternal::Color(self.clone()))
    }
}

#[derive(Clone)]
pub struct DepthAttachment {
    raw: raw::Attachment,
}

impl DepthAttachment {
    pub(super) fn from_raw(raw: raw::Attachment) -> Self {
        Self { raw }
    }
}

impl<'a> Into<Framebuffer<()>> for &'a DepthAttachment {
    fn into(self) -> Framebuffer<()> {
        Framebuffer(FramebufferInternal::Depth(self.clone()))
    }
}

#[derive(Clone)]
pub struct ColorDepthFramebuffer<F: Fragment<Sl>> {
    pub color: F::Gl,
    pub depth: DepthAttachment,
}

impl<'a, F: Fragment<Sl>> Into<Framebuffer<F>> for &'a ColorDepthFramebuffer<F> {
    fn into(self) -> Framebuffer<F> {
        Framebuffer(FramebufferInternal::ColorDepth(self.clone()))
    }
}

#[derive(Clone)]
enum FramebufferInternal<F: Fragment<Sl>> {
    Default,
    Depth(DepthAttachment),
    Color(F::Gl),
    ColorDepth(ColorDepthFramebuffer<F>),
}

#[derive(Clone)]
pub struct Framebuffer<F: Fragment<Sl> = sl::Vec4>(FramebufferInternal<F>);

impl<F: Fragment<Sl>> Framebuffer<F> {
    pub fn raw(&self) -> raw::Framebuffer {
        use FramebufferInternal::*;

        match &self.0 {
            Default => raw::Framebuffer::Default,
            Depth(depth) => raw::Framebuffer::Attachments {
                attachments: vec![depth.raw.clone()],
            },
            Color(color) => raw::Framebuffer::Attachments {
                attachments: raw_color_attachments(color),
            },
            ColorDepth(color_depth) => {
                let mut attachments = raw_color_attachments(&color_depth.color);
                attachments.push(color_depth.depth.raw.clone());
                raw::Framebuffer::Attachments { attachments }
            }
        }
    }
}

impl Default for Framebuffer<sl::Vec4> {
    fn default() -> Self {
        Framebuffer(FramebufferInternal::Default)
    }
}

impl<'a, F: Fragment<Sl>> Into<Framebuffer<F>> for &'a Framebuffer<F> {
    fn into(self) -> Framebuffer<F> {
        self.clone()
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
