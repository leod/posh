use std::marker::PhantomData;

use crate::sl::{self, Sample};

use super::raw::{self, ImageComponentType, ImageInternalFormat};

pub struct Image<'a, S> {
    raw: raw::Image<'a>,
    _phantom: PhantomData<S>,
}

impl<'a, S: Sample> Image<'a, S> {
    pub fn raw(self) -> raw::Image<'a> {
        self.raw
    }
}

impl<'a> Image<'a, sl::Vec4> {
    pub fn slice_u8(size: glam::UVec2, data: &'a [u8]) -> Self {
        Image {
            raw: raw::Image {
                size,
                ty: ImageComponentType::U8,
                internal_format: ImageInternalFormat::RgbaU8,
                data: Some(data),
            },
            _phantom: PhantomData,
        }
    }

    pub fn slice_u8_srgb(size: glam::UVec2, data: &'a [u8]) -> Self {
        Image {
            raw: raw::Image {
                size,
                ty: ImageComponentType::U8,
                internal_format: ImageInternalFormat::SrgbU8AlphaU8,
                data: Some(data),
            },
            _phantom: PhantomData,
        }
    }

    pub fn slice_i8_snorm(size: glam::UVec2, data: &'a [i8]) -> Self {
        Image {
            raw: raw::Image {
                size,
                ty: ImageComponentType::I8,
                internal_format: ImageInternalFormat::RgbaI8Snorm,
                data: Some(bytemuck::cast_slice(data)),
            },
            _phantom: PhantomData,
        }
    }

    pub fn slice_f32(size: glam::UVec2, data: &'a [f32]) -> Self {
        Image {
            raw: raw::Image {
                size,
                ty: ImageComponentType::F32,
                internal_format: ImageInternalFormat::RgbaF32,
                data: Some(bytemuck::cast_slice(data)),
            },
            _phantom: PhantomData,
        }
    }

    pub fn zeroed_u8(size: glam::UVec2) -> Self {
        Image {
            raw: raw::Image {
                size,
                ty: ImageComponentType::U8,
                internal_format: ImageInternalFormat::RgbaU8,
                data: None,
            },
            _phantom: PhantomData,
        }
    }

    pub fn zeroed_u8_srgb(size: glam::UVec2) -> Self {
        Image {
            raw: raw::Image {
                size,
                ty: ImageComponentType::U8,
                internal_format: ImageInternalFormat::SrgbU8AlphaU8,
                data: None,
            },
            _phantom: PhantomData,
        }
    }

    pub fn zeroed_i8_snorm(size: glam::UVec2) -> Self {
        Image {
            raw: raw::Image {
                size,
                ty: ImageComponentType::I8,
                internal_format: ImageInternalFormat::RgbaI8Snorm,
                data: None,
            },
            _phantom: PhantomData,
        }
    }

    pub fn zeroed_f32(size: glam::UVec2) -> Self {
        Image {
            raw: raw::Image {
                size,
                ty: ImageComponentType::F32,
                internal_format: ImageInternalFormat::RgbaF32,
                data: None,
            },
            _phantom: PhantomData,
        }
    }
}

// TODO:
// - RgbaIntFormat
// - RgbaUnsignedIntFormat
// - RgbFormat
// - RgbIntFormat
// - RgbUnsignedIntFormat
// - RgFormat
// - RgIntFormat
// - RgUnsignedIntFormat
// - RedFormat
// - RedIntFormat
// - RedUnsignedIntFormat
// - DepthFormat
// - DepthStencilFormat

/*

RGBA - UNSIGNED_BYTE - RGBA8, SRGB8_ALPHA8
RGBA - BYTE          - RGBA8_SNORM
RGBA - FLOAT         - RGBA32F, RGBA16F

RGB  - UNSIGNED_BYTE - RGB8, SRGB8
RGB  - BYTE          - RGB8_SNORM
RGB  - FLOAT         - RGB32, RGB16F

RG   - UNSIGNED_BYTE - RG8
RG   - BYTE          - RG8_SNORM
RG   - FLOAT         - RG32F, RG16F

RED  - UNSIGNED_BYTE - R8
RED  - BYTE          - R8_SNORM
RED  - FLOAT         - R32F, R16F

RGBA_INTEGER - UNSIGNED_BYTE - RGBA8UI
RGBA_INTEGER - BYTE          - RGBA8I
...

RGB_INTEGER  - UNSIGNED_BYTE  - RGB8UI
RGB_INTEGER  - BYTE           - RGB8I
...

RG_INTEGER   - UNSIGNED_BYTE  - RG8UI
RG_INTEGER   - BYTE           - RG 8I
...

RED_INTEGER  - UNSIGNED_BYTE  - R8UI
RED_INTEGER  - BYTE           - R8I
...

DEPTH_COMPONENT - UNSIGNED_SHORT - DEPTH_COMPONENT16
DEPTH_COMPONENT - UNSIGNED_INT   - DEPTH_COMPONENT24
DEPTH_COMPONENT - FLOAT          - DEPTH_COMPONENT32F

DEPTH_STENCIL   - UNSIGNED_INT_24_8 - DEPTH24_STENCIL8

*/
