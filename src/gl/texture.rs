use std::{marker::PhantomData, rc::Rc};

use super::{
    raw::{self, Sampler2dParams},
    ImageFormat,
};

#[derive(Clone)]
pub struct Texture2d<Format: ImageFormat> {
    raw: Rc<raw::Texture2d>,
    _phantom: PhantomData<Format>,
}

impl<Format: ImageFormat> Texture2d<Format> {
    pub(super) fn from_raw(raw: raw::Texture2d) -> Self {
        assert!(Format::ALLOWED_INTERNAL_FORMATS.contains(&raw.internal_format()));

        Self {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }

    pub fn binding(&self, params: Sampler2dParams) -> Sampler2d {
        Sampler2d {
            raw: self.raw.sampler(params),
        }
    }
}

pub struct Sampler2d {
    pub(super) raw: raw::Sampler2d,
}
