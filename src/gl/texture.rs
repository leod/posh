use std::{marker::PhantomData, rc::Rc};

use crate::{gl::raw::ImageInternalFormat, sl::Sample};

use super::{
    raw::{self, Sampler2dParams},
    FramebufferAttachment2d,
};

pub struct Texture2d<S> {
    raw: Rc<raw::Texture2d>,
    _phantom: PhantomData<S>,
}

pub struct Sampler2d<S> {
    raw: raw::Sampler2d,
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

    pub fn attachment(&self) -> FramebufferAttachment2d<S> {
        self.attachment_with_level(0)
    }

    pub fn attachment_with_level(&self, level: u32) -> FramebufferAttachment2d<S> {
        FramebufferAttachment2d::from_raw(raw::FramebufferAttachment2d {
            texture: self.raw.clone(),
            level,
        })
    }

    pub fn sampler(&self, params: Sampler2dParams) -> Sampler2d<S> {
        Sampler2d::from_raw(raw::Sampler2d {
            texture: self.raw.clone(),
            params,
        })
    }
}

impl<S> Sampler2d<S> {
    pub(super) fn from_raw(raw: raw::Sampler2d) -> Self {
        Self {
            raw,
            _phantom: PhantomData,
        }
    }

    pub fn raw(&self) -> &raw::Sampler2d {
        &self.raw
    }
}
