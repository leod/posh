use std::marker::PhantomData;

use crate::{
    interface::FragmentVisitor,
    sl::{self, Sample},
    Fragment, GlView, SlView,
};

use super::{
    raw::{self, FramebufferError},
    Sampler2dParams, Texture2d, Texture2dBinding,
};

pub struct FramebufferAttachment2d<S> {
    pub(super) texture: Texture2d<S>,
    pub(super) level: u32,
}

pub struct Framebuffer<F: Fragment<SlView>> {
    raw: raw::Framebuffer,
    attachments: F::GlView,
}

impl<F: Fragment<SlView>> Framebuffer<F> {}

pub struct FramebufferBinding<F> {
    raw: raw::FramebufferBinding,
    _phantom: PhantomData<F>,
}

impl<S: Sample> FramebufferAttachment2d<S> {
    pub(super) fn raw(&self) -> raw::FramebufferAttachment {
        raw::FramebufferAttachment::Texture2d {
            texture: self.texture.raw(),
            level: self.level,
        }
    }

    pub fn texture(&self) -> &Texture2d<S> {
        &self.texture
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn binding(&self, params: Sampler2dParams) -> Texture2dBinding<S> {
        self.texture.binding(params)
    }
}

impl<F: Fragment<SlView>> Framebuffer<F> {
    pub(super) fn new(
        context: &raw::Context,
        attachments: F::GlView,
    ) -> Result<Self, FramebufferError> {
        let raw_attachments = raw_attachments(&attachments);
        let raw = context.create_framebuffer(&raw_attachments)?;

        Ok(Self { raw, attachments })
    }

    pub fn attachments(&self) -> &F::GlView {
        &self.attachments
    }

    pub fn binding(&self) -> FramebufferBinding<F::GlView> {
        FramebufferBinding {
            raw: self.raw.binding(),
            _phantom: PhantomData,
        }
    }
}

impl Default for FramebufferBinding<FramebufferAttachment2d<sl::Vec4>> {
    fn default() -> Self {
        Self {
            raw: raw::FramebufferBinding::Default,
            _phantom: PhantomData,
        }
    }
}

impl<F: Fragment<GlView>> FramebufferBinding<F> {
    pub(super) fn raw(&self) -> &raw::FramebufferBinding {
        &self.raw
    }
}

fn raw_attachments<F: Fragment<GlView>>(attachments: &F) -> Vec<raw::FramebufferAttachment> {
    struct Visitor<'a>(Vec<raw::FramebufferAttachment<'a>>);
    impl<'a> FragmentVisitor<'a, GlView> for Visitor<'a> {
        fn accept<S: Sample>(&mut self, path: &str, attachment: &'a FramebufferAttachment2d<S>) {
            self.0.push(attachment.raw());
        }
    }

    // TODO: Remove hardcoded path names.
    let mut visitor = Visitor(Vec::new());
    attachments.visit("fragment_output", &mut visitor);

    visitor.0
}
