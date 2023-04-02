use std::{marker::PhantomData, rc::Rc};

use crate::sl::{self, ColorSample};

use super::{
    raw::{self, Sampler2dParams},
    ColorAttachment, ColorImage, Comparison, DepthAttachment, DepthImage, TextureError,
};

pub struct ColorTexture2d<S = sl::Vec4> {
    raw: Rc<raw::Texture2d>,
    _phantom: PhantomData<S>,
}

pub struct DepthTexture2d {
    raw: Rc<raw::Texture2d>,
}

#[derive(Clone)]
pub struct ColorSampler2d<S = sl::Vec4> {
    raw: raw::Sampler2d,
    _phantom: PhantomData<S>,
}

#[derive(Clone)]
pub struct ComparisonSampler2d {
    raw: raw::Sampler2d,
}

impl<S> ColorTexture2d<S> {
    pub(super) fn from_raw(raw: raw::Texture2d) -> Self {
        // FIXME: This should validate against `S`.

        Self {
            raw: Rc::new(raw),
            _phantom: PhantomData,
        }
    }
}

impl<S: ColorSample> ColorTexture2d<S> {
    pub fn as_color_attachment(&self) -> ColorAttachment<S> {
        self.as_color_attachment_with_level(0)
    }

    pub fn as_color_attachment_with_level(&self, level: u32) -> ColorAttachment<S> {
        ColorAttachment::from_raw(raw::Attachment::Texture2d {
            texture: self.raw.clone(),
            level,
        })
    }

    pub fn as_color_sampler(&self, params: Sampler2dParams) -> ColorSampler2d<S> {
        // FIXME: Check texture completeness.
        ColorSampler2d::from_raw(raw::Sampler2d {
            texture: self.raw.clone(),
            params,
            comparison: None,
        })
    }

    pub fn set(
        &self,
        level: usize,
        lower_left_corner: glam::UVec2,
        image: ColorImage<S>,
    ) -> Result<(), TextureError> {
        self.raw.set(level, lower_left_corner, image.raw())
    }
}

impl DepthTexture2d {
    pub(super) fn from_raw(raw: raw::Texture2d) -> Self {
        // FIXME: This should validate against depth.

        Self { raw: Rc::new(raw) }
    }
}

impl DepthTexture2d {
    pub fn as_depth_attachment(&self) -> DepthAttachment {
        self.as_depth_attachment_with_level(0)
    }

    pub fn as_depth_attachment_with_level(&self, level: u32) -> DepthAttachment {
        DepthAttachment::from_raw(raw::Attachment::Texture2d {
            texture: self.raw.clone(),
            level,
        })
    }

    pub fn as_color_sampler(&self, params: Sampler2dParams) -> ColorSampler2d<sl::F32> {
        // FIXME: Check texture completeness.
        ColorSampler2d::from_raw(raw::Sampler2d {
            texture: self.raw.clone(),
            params,
            comparison: None,
        })
    }

    pub fn as_comparison_sampler(
        &self,
        params: Sampler2dParams,
        comparison: Comparison,
    ) -> ComparisonSampler2d {
        // FIXME: Check texture completeness.
        ComparisonSampler2d::from_raw(raw::Sampler2d {
            texture: self.raw.clone(),
            params,
            comparison: Some(comparison),
        })
    }

    pub fn set(
        &self,
        level: usize,
        lower_left_corner: glam::UVec2,
        image: DepthImage,
    ) -> Result<(), TextureError> {
        self.raw.set(level, lower_left_corner, image.raw())
    }
}

impl<S> ColorSampler2d<S> {
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

impl ComparisonSampler2d {
    fn from_raw(raw: raw::Sampler2d) -> Self {
        Self { raw }
    }

    pub fn raw(&self) -> &raw::Sampler2d {
        &self.raw
    }
}
