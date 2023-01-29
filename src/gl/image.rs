use sealed::sealed;

#[doc(hidden)]
pub struct RawImageData<'a> {
    size: [usize; 2],
    ty: u32,
    internal_format: u32,
    data: &'a [u8],
}

#[sealed]
pub trait ImageData {
    #[doc(hidden)]
    fn raw(&self) -> &RawImageData;
}

#[sealed]
pub trait ImageFormat {
    #[doc(hidden)]
    const FORMAT: u32;

    type Data<'a>: ImageData;
}

pub struct RgbaData<'a>(RawImageData<'a>);

#[sealed]
impl<'a> ImageData for RgbaData<'a> {
    fn raw(&self) -> &RawImageData {
        &self.0
    }
}

impl<'a> RgbaData<'a> {
    pub fn from_u8(size: [usize; 2], data: &'a [u8]) -> Self {
        // Safety: check that `data` has the correct size.
        assert_eq!(data.len(), size[0] * size[1] * 4);

        RgbaData(RawImageData {
            size,
            ty: glow::UNSIGNED_BYTE,
            internal_format: glow::RGBA8,
            data,
        })
    }

    pub fn from_u8_srgb(size: [usize; 2], data: &'a [u8]) -> Self {
        // Safety: check that `data` has the correct size.
        assert_eq!(data.len(), size[0] * size[1] * 4);

        RgbaData(RawImageData {
            size,
            ty: glow::UNSIGNED_BYTE,
            internal_format: glow::SRGB8_ALPHA8,
            data,
        })
    }

    pub fn from_i8_snorm(size: [usize; 2], data: &'a [i8]) -> Self {
        // Safety: check that `data` has the correct size.
        assert_eq!(data.len(), size[0] * size[1] * 4);

        RgbaData(RawImageData {
            size,
            ty: glow::BYTE,
            internal_format: glow::RGBA8_SNORM,
            data: bytemuck::cast_slice(data),
        })
    }

    pub fn from_f32(size: [usize; 2], data: &'a [f32]) -> Self {
        // Safety: check that `data` has the correct size.
        assert_eq!(data.len(), size[0] * size[1] * 4);

        RgbaData(RawImageData {
            size,
            ty: glow::FLOAT,
            internal_format: glow::RGBA32F,
            data: bytemuck::cast_slice(data),
        })
    }
}

pub struct Rgba;

#[sealed]
impl ImageFormat for Rgba {
    const FORMAT: u32 = glow::RGBA;

    type Data<'a> = RgbaData<'a>;
}

pub enum RgbaInt {}

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
