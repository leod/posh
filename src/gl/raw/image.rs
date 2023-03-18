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
            Depth => glow::DEPTH_COMPONENT,
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

// FIXME: When is `ImageComponentType` not implied by `ImageInternalFormat`?
// Looks like the internal format `DepthComponent16` can be used with both `U16`
// and `U32` (OpenGL ES 3.0.6, Table 3.2)?
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImageComponentType {
    U8,
    I8,
    F32,
}

impl ImageComponentType {
    pub const fn to_gl(self) -> u32 {
        use ImageComponentType::*;

        match self {
            U8 => glow::UNSIGNED_BYTE,
            I8 => glow::BYTE,
            F32 => glow::FLOAT,
        }
    }

    pub const fn size_of(self) -> usize {
        use ImageComponentType::*;

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
    DepthF32,
}

impl ImageInternalFormat {
    pub const fn to_gl(self) -> u32 {
        use ImageInternalFormat::*;

        match self {
            RgbaU8 => glow::RGBA8,
            SrgbU8AlphaU8 => glow::SRGB8_ALPHA8,
            RgbaI8Snorm => glow::RGBA8_SNORM,
            RgbaF32 => glow::RGBA32F,
            DepthF32 => glow::DEPTH_COMPONENT32F,
        }
    }

    pub const fn to_format(self) -> ImageFormat {
        use ImageInternalFormat::*;

        match self {
            RgbaU8 | SrgbU8AlphaU8 | RgbaI8Snorm | RgbaF32 => ImageFormat::Rgba,
            DepthF32 => ImageFormat::Depth,
        }
    }

    pub fn matches_type(self, ty: ImageComponentType) -> bool {
        use ImageInternalFormat::*;

        match self {
            RgbaU8 => ty == ImageComponentType::U8,
            SrgbU8AlphaU8 => ty == ImageComponentType::U8,
            RgbaI8Snorm => ty == ImageComponentType::I8,
            RgbaF32 => ty == ImageComponentType::F32,
            DepthF32 => ty == ImageComponentType::F32,
        }
    }

    pub fn is_color_renderable(&self) -> bool {
        use ImageInternalFormat::*;

        match self {
            RgbaU8 | SrgbU8AlphaU8 | RgbaI8Snorm | RgbaF32 => true,
            DepthF32 => false,
        }
    }

    pub fn is_depth_renderable(&self) -> bool {
        use ImageInternalFormat::*;

        match self {
            RgbaU8 | SrgbU8AlphaU8 | RgbaI8Snorm | RgbaF32 => false,
            DepthF32 => true,
        }
    }

    pub fn is_stencil_renderable(&self) -> bool {
        use ImageInternalFormat::*;

        match self {
            RgbaU8 | SrgbU8AlphaU8 | RgbaI8Snorm | RgbaF32 => false,
            DepthF32 => false,
        }
    }
}

#[doc(hidden)]
#[derive(Clone)]
pub struct Image<'a> {
    pub size: glam::UVec2,
    pub ty: ImageComponentType,
    pub internal_format: ImageInternalFormat,
    pub data: Option<&'a [u8]>,
}

impl<'a> Image<'a> {
    pub fn required_data_len(&self) -> usize {
        let width = self.size.x as usize;
        let height = self.size.y as usize;
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
