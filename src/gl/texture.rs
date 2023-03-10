use std::{marker::PhantomData, rc::Rc};

use crate::{gl::raw::ImageInternalFormat, sl::Sample};

use super::{
    raw::{self, Sampler2dParams},
    FramebufferAttachment2d,
};

#[derive(Clone)]
pub struct Texture2d<S> {
    raw: Rc<raw::Texture2d>,
    _phantom: PhantomData<S>,
}

impl<S: Sample> Texture2d<S> {
    pub(super) fn from_raw(raw: raw::Texture2d) -> Self {
        use ImageInternalFormat::*;

        // FIXME: This should validate against `S`.
        assert!([RgbaU8, SrgbU8AlphaU8, RgbaI8Snorm, RgbaF32].contains(&raw.internal_format()));

        Self {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub(super) fn raw(&self) -> &raw::Texture2d {
        &self.raw
    }

    pub fn attachment(&self) -> FramebufferAttachment2d<S> {
        self.attachment_with_level(0)
    }

    pub fn attachment_with_level(&self, level: u32) -> FramebufferAttachment2d<S> {
        FramebufferAttachment2d {
            texture: self.clone(),
            level,
        }
    }

    pub fn binding(&self, params: Sampler2dParams) -> Texture2dBinding<S> {
        Texture2dBinding {
            raw: self.raw.binding(params),
            _phantom: PhantomData,
        }
    }
}

pub struct Texture2dBinding<S> {
    raw: raw::Texture2dBinding,
    _phantom: PhantomData<S>,
}

impl<S> Texture2dBinding<S> {
    pub fn raw(&self) -> &raw::Texture2dBinding {
        &self.raw
    }
}
