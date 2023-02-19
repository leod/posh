use std::{marker::PhantomData, rc::Rc};

use crate::{gl::raw::ImageInternalFormat, sl::Sample};

use super::raw::{self, Sampler2dParams};

#[derive(Clone)]
pub struct Texture2d<S> {
    raw: Rc<raw::Texture2d>,
    _phantom: PhantomData<S>,
}

impl<S: Sample> Texture2d<S> {
    pub(super) fn from_raw(raw: raw::Texture2d) -> Self {
        use ImageInternalFormat::*;

        assert!([RgbaU8, SrgbU8AlphaU8, RgbaI8Snorm, RgbaF32].contains(&raw.internal_format()));

        Self {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub fn binding(&self, params: Sampler2dParams) -> Sampler2d<S> {
        Sampler2d {
            raw: self.raw.sampler(params),
            _phantom: PhantomData,
        }
    }
}

pub struct Sampler2d<S> {
    pub(super) raw: raw::Sampler2d,
    _phantom: PhantomData<S>,
}
