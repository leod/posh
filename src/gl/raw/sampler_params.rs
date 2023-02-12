use super::ComparisonFunc;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SamplerMagFilter {
    Nearest,
    Linear,
}

impl SamplerMagFilter {
    pub const fn to_gl(self) -> u32 {
        use SamplerMagFilter::*;

        match self {
            Nearest => glow::NEAREST,
            Linear => glow::LINEAR,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SamplerMinFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapNearest,
    LinearMipmapLinear,
}

impl SamplerMinFilter {
    pub const fn to_gl(self) -> u32 {
        use SamplerMinFilter::*;

        match self {
            Nearest => glow::NEAREST,
            Linear => glow::LINEAR,
            NearestMipmapNearest => glow::NEAREST_MIPMAP_NEAREST,
            NearestMipmapLinear => glow::NEAREST_MIPMAP_LINEAR,
            LinearMipmapNearest => glow::LINEAR_MIPMAP_NEAREST,
            LinearMipmapLinear => glow::LINEAR_MIPMAP_LINEAR,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SamplerWrap {
    ClampToEdge,
    Repeat,
    MirroredRepeat,
}

impl SamplerWrap {
    pub const fn to_gl(self) -> u32 {
        use SamplerWrap::*;

        match self {
            ClampToEdge => glow::CLAMP_TO_EDGE,
            Repeat => glow::REPEAT,
            MirroredRepeat => glow::MIRRORED_REPEAT,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Sampler2dParams {
    pub comparison_func: Option<ComparisonFunc>,
    pub mag_filter: SamplerMagFilter,
    pub min_filter: SamplerMinFilter,
    pub wrap_s: SamplerWrap,
    pub wrap_t: SamplerWrap,
}

impl Default for Sampler2dParams {
    fn default() -> Self {
        Self {
            comparison_func: None,
            mag_filter: SamplerMagFilter::Linear,
            min_filter: SamplerMinFilter::NearestMipmapLinear,
            wrap_s: SamplerWrap::Repeat,
            wrap_t: SamplerWrap::Repeat,
        }
    }
}
