use std::rc::Rc;

use sealed::sealed;

use crate::{Block, Sl, ToSl};

use super::{
    dag::{BuiltInType, Expr, SamplerType, Trace, Type},
    primitives::built_in_2,
    IVec2, IVec3, IVec4, Interpolant, Object, UVec2, UVec3, UVec4, Value, Vec2, Vec3, Vec4, F32,
    I32, U32,
};

// FIXME: These traits desperately need to move to `interface`.

#[sealed]
pub trait Sample {
    const SAMPLER_TYPE: SamplerType;

    type Vec4: Value;

    type Gl;

    fn from_vec4(v: Self::Vec4) -> Self;
}

#[sealed]
pub trait ColorSample: Value + Interpolant + Sample {}

macro_rules! impl_color_sample {
    ($sample:ident, $ty:ident, |$vec4_name:ident: $vec4:ident| $from_vec4:expr) => {
        #[sealed]
        impl Sample for $sample {
            const SAMPLER_TYPE: SamplerType = SamplerType::$ty;

            type Vec4 = $vec4;

            type Gl = <$sample as Block<Sl>>::Gl;

            fn from_vec4($vec4_name: Self::Vec4) -> Self {
                $from_vec4
            }
        }

        #[sealed]
        impl ColorSample for $sample {}
    };
}

impl_color_sample!(F32, ColorSampler2d, |v: Vec4| v.x);
impl_color_sample!(I32, IColorSampler2d, |v: IVec4| v.x);
impl_color_sample!(U32, UColorSampler2d, |v: UVec4| v.x);
impl_color_sample!(Vec2, ColorSampler2d, |v: Vec4| v.xy());
impl_color_sample!(IVec2, IColorSampler2d, |v: IVec4| v.xy());
impl_color_sample!(UVec2, UColorSampler2d, |v: UVec4| v.xy());
impl_color_sample!(Vec3, ColorSampler2d, |v: Vec4| v.xyz());
impl_color_sample!(IVec3, IColorSampler2d, |v: IVec4| v.xyz());
impl_color_sample!(UVec3, UColorSampler2d, |v: UVec4| v.xyz());
impl_color_sample!(Vec4, ColorSampler2d, |v: Vec4| v);
impl_color_sample!(IVec4, IColorSampler2d, |v: IVec4| v);
impl_color_sample!(UVec4, UColorSampler2d, |v: UVec4| v);

#[derive(Debug, Copy, Clone)]
pub enum ColorSampler2dInternal<S> {
    Sampler(Trace),
    Constant(S),
}

/// An object which can be sampled.
#[derive(Debug, Copy, Clone)]
pub struct ColorSampler2d<S = Vec4>(ColorSampler2dInternal<S>);

#[derive(Debug, Copy, Clone)]
pub struct ComparisonSampler2d {
    trace: Trace,
}

impl<S: ColorSample> Object for ColorSampler2d<S> {
    fn ty() -> Type {
        Type::BuiltIn(BuiltInType::Sampler(S::SAMPLER_TYPE))
    }

    fn expr(&self) -> Rc<Expr> {
        use ColorSampler2dInternal::*;

        match &self.0 {
            Sampler(sampler) => sampler.expr(),
            Constant(sample) => sample.expr(),
        }
    }

    fn from_arg(name: &str) -> Self {
        ColorSampler2d(ColorSampler2dInternal::Sampler(Trace::new(Expr::Arg {
            ty: Self::ty(),
            name: name.into(),
        })))
    }
}

impl<S: ColorSample> ColorSampler2d<S> {
    pub fn constant(sample: S) -> Self {
        ColorSampler2d(ColorSampler2dInternal::Constant(sample))
    }

    pub fn sample(self, tex_coords: Vec2) -> S {
        let sample = built_in_2("texture", self, tex_coords);

        S::from_vec4(sample)
    }

    pub fn size(self, level: impl ToSl<Output = U32>) -> UVec2 {
        let size: IVec2 = built_in_2("textureSize", self, level.to_sl().as_i32());

        size.as_uvec2()
    }

    // TODO: Various sampling functions.
}

impl Object for ComparisonSampler2d {
    fn ty() -> Type {
        Type::BuiltIn(BuiltInType::Sampler(SamplerType::ComparisonSampler2d))
    }

    fn expr(&self) -> Rc<Expr> {
        self.trace.expr()
    }

    fn from_arg(name: &str) -> Self {
        Self {
            trace: Trace::new(Expr::Arg {
                ty: Self::ty(),
                name: name.into(),
            }),
        }
    }
}

impl ComparisonSampler2d {
    pub fn sample_compare(self, tex_coords: Vec2, reference_depth: F32) -> F32 {
        built_in_2("texture", self, tex_coords.extend(reference_depth))
    }

    pub fn size(self, level: impl ToSl<Output = U32>) -> UVec2 {
        let size: IVec2 = built_in_2("textureSize", self, level.to_sl().as_i32());

        size.as_uvec2()
    }
}
