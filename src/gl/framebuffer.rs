use std::{marker::PhantomData, rc::Rc};

use crate::{
    interface::FragmentVisitor,
    sl::{self, Sample},
    Fragment, GlView, SlView,
};

use super::{
    raw::{self, FramebufferError},
    Sampler2d, Sampler2dParams,
};

pub struct FramebufferAttachment2d<S> {
    raw: raw::FramebufferAttachment2d,
    _phantom: PhantomData<S>,
}

pub struct Framebuffer<F: Fragment<SlView>> {
    raw: Rc<raw::Framebuffer>,
    attachments: F::GlView,
}

impl<F: Fragment<SlView>> Framebuffer<F> {}

#[derive(Clone)]
pub struct FramebufferBinding<F> {
    raw: raw::FramebufferBinding,
    _phantom: PhantomData<F>,
}

impl<S: Sample> FramebufferAttachment2d<S> {
    pub(super) fn from_raw(raw: raw::FramebufferAttachment2d) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }

    pub fn sampler(&self, params: Sampler2dParams) -> Sampler2d<S> {
        Sampler2d::from_raw(raw::Sampler2d {
            texture: self.raw.texture.clone(),
            params,
        })
    }

    pub fn level(&self) -> u32 {
        self.raw.level
    }
}

impl<F: Fragment<SlView>> Framebuffer<F> {
    pub(super) fn new(
        context: &raw::Context,
        attachments: F::GlView,
    ) -> Result<Self, FramebufferError> {
        let raw_attachments = raw_attachments(&attachments);
        let raw = context.create_framebuffer(&raw_attachments)?;

        Ok(Self {
            raw: Rc::new(raw),
            attachments,
        })
    }

    pub fn attachments(&self) -> &F::GlView {
        &self.attachments
    }

    pub fn binding(&self) -> FramebufferBinding<F::GlView> {
        FramebufferBinding {
            raw: raw::FramebufferBinding::Framebuffer(self.raw.clone()),
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
    struct Visitor(Vec<raw::FramebufferAttachment>);
    impl<'a> FragmentVisitor<'a, GlView> for Visitor {
        fn accept<S: Sample>(&mut self, _: &str, attachment: &FramebufferAttachment2d<S>) {
            self.0.push(raw::FramebufferAttachment::Texture2d(
                attachment.raw.clone(),
            ));
        }
    }

    // TODO: Remove hardcoded path names.
    let mut visitor = Visitor(Vec::new());
    attachments.visit("fragment_output", &mut visitor);

    visitor.0
}
