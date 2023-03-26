use glow::HasContext;

use super::CompareFunc;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SamplerMagFilter {
    Nearest,
    Linear,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SamplerWrap {
    ClampToEdge,
    Repeat,
    MirroredRepeat,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Sampler2dParams {
    pub mag_filter: SamplerMagFilter,
    pub min_filter: SamplerMinFilter,
    pub wrap_s: SamplerWrap,
    pub wrap_t: SamplerWrap,
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

impl Default for Sampler2dParams {
    fn default() -> Self {
        Self {
            mag_filter: SamplerMagFilter::Linear,
            min_filter: SamplerMinFilter::NearestMipmapLinear,
            wrap_s: SamplerWrap::Repeat,
            wrap_t: SamplerWrap::Repeat,
        }
    }
}

impl Sampler2dParams {
    pub(super) fn set_delta(&self, gl: &glow::Context, current: &Sampler2dParams) {
        if self.mag_filter != current.mag_filter {
            let mag_filter = self.mag_filter.to_gl() as i32;

            unsafe {
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, mag_filter);
            }
        }

        if self.min_filter != current.min_filter {
            let min_filter = self.min_filter.to_gl() as i32;

            unsafe {
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, min_filter);
            }
        }

        if self.wrap_s != current.wrap_s {
            let wrap_s = self.wrap_s.to_gl() as i32;

            unsafe {
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, wrap_s);
            }
        }

        if self.wrap_t != current.wrap_t {
            let wrap_t = self.wrap_t.to_gl() as i32;

            unsafe {
                gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, wrap_t);
            }
        }
    }
}

pub(super) fn set_comparison_func(gl: &glow::Context, target: u32, compare: Option<CompareFunc>) {
    let mode = compare.map_or(glow::NONE, |_| glow::COMPARE_REF_TO_TEXTURE) as i32;
    unsafe { gl.tex_parameter_i32(target, glow::TEXTURE_COMPARE_MODE, mode) };

    if let Some(compare) = compare {
        let func = compare.to_gl() as i32;
        unsafe { gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_COMPARE_FUNC, func) };
    }
}
