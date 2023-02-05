#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageFormat {
    Rgba,
    Rgb,
    Rg,
    Red,
    RgbaInteger,
    RgbInteger,
    RgInteger,
    RedInteger,
    Depth,
    DepthStencil,
}

impl ImageFormat {
    pub const fn to_gl(self) -> u32 {
        use ImageFormat::*;

        match self {
            Rgba => glow::RGBA,
            Rgb => glow::RGB,
            Rg => glow::RG,
            Red => glow::RED,
            RgbaInteger => glow::RGBA_INTEGER,
            RgbInteger => glow::RGB_INTEGER,
            RgInteger => glow::RG_INTEGER,
            RedInteger => glow::RED_INTEGER,
            Depth => glow::DEPTH,
            DepthStencil => glow::DEPTH_STENCIL,
        }
    }

    pub const fn size(self) -> usize {
        use ImageFormat::*;

        match self {
            Rgba => 4,
            Rgb => 3,
            Rg => 2,
            Red => 1,
            RgbaInteger => 4,
            RgbInteger => 3,
            RgInteger => 2,
            RedInteger => 1,
            Depth => 1,
            DepthStencil => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageType {
    U8,
    I8,
    F32,
}

impl ImageType {
    pub const fn to_gl(self) -> u32 {
        use ImageType::*;

        match self {
            U8 => glow::UNSIGNED_BYTE,
            I8 => glow::BYTE,
            F32 => glow::FLOAT,
        }
    }

    pub const fn size_of(self) -> usize {
        use ImageType::*;

        match self {
            U8 => 1,
            I8 => 1,
            F32 => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageInternalFormat {
    RgbaU8,
    SrgbU8AlphaU8,
    RgbaI8Snorm,
    RgbaF32,
}

impl ImageInternalFormat {
    pub const fn to_gl(self) -> u32 {
        use ImageInternalFormat::*;

        match self {
            RgbaU8 => glow::RGBA8,
            SrgbU8AlphaU8 => glow::SRGB8_ALPHA8,
            RgbaI8Snorm => glow::RGBA8_SNORM,
            RgbaF32 => glow::RGBA32F,
        }
    }

    pub const fn to_format(self) -> ImageFormat {
        use ImageInternalFormat::*;

        match self {
            RgbaU8 | SrgbU8AlphaU8 | RgbaI8Snorm | RgbaF32 => ImageFormat::Rgba,
        }
    }

    pub fn matches_type(self, ty: ImageType) -> bool {
        use ImageInternalFormat::*;

        match self {
            RgbaU8 => ty == ImageType::U8,
            SrgbU8AlphaU8 => ty == ImageType::U8,
            RgbaI8Snorm => ty == ImageType::I8,
            RgbaF32 => ty == ImageType::F32,
        }
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub struct Image<'a> {
    pub dimensions: (u32, u32),
    pub ty: ImageType,
    pub internal_format: ImageInternalFormat,
    pub data: Option<&'a [u8]>,
}

impl<'a> Image<'a> {
    pub fn required_data_len(&self) -> usize {
        let width = usize::try_from(self.dimensions.0).unwrap();
        let height = usize::try_from(self.dimensions.0).unwrap();
        let bytes = self.ty.size_of();
        let num_components = self.internal_format.to_format().size();

        width
            .checked_mul(height)
            .unwrap()
            .checked_mul(bytes)
            .unwrap()
            .checked_mul(num_components)
            .unwrap()
    }
}
