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

        // FIXME: This should validate against `S`.
        assert!([RgbaU8, SrgbU8AlphaU8, RgbaI8Snorm, RgbaF32].contains(&raw.internal_format()));

        Self {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub fn attachment(&self) -> Self {
        todo!()
    }

    pub fn attachment_with_level(&self, level: usize) -> Self {
        todo!()
    }

    pub fn binding(&self, params: Sampler2dParams) -> Texture2dBinding<S> {
        Texture2dBinding {
            raw: self.raw.binding(params),
            _phantom: PhantomData,
        }
    }
}

pub struct Texture2dBinding<S> {
    pub(super) raw: raw::Texture2dBinding,
    _phantom: PhantomData<S>,
}
