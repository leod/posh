use std::{marker::PhantomData, rc::Rc};

use sealed::sealed;

use crate::{Block, Sl};

use super::{
    dag::{BuiltInType, Expr, SamplerType, Trace, Type},
    primitives::built_in_2,
    IVec2, IVec3, IVec4, Object, UVec2, UVec3, UVec4, Value, Vec2, Vec3, Vec4, F32, I32, U32,
};

// FIXME: These traits desparately needs to move to `interface`

#[sealed]
pub trait Sample {
    const SAMPLER_TYPE: SamplerType;

    type Gl;
}

#[sealed]
pub trait ColorSample: Value + Sample {}

macro_rules! impl_color_sample {
    ($sample:ident, $ty:ident) => {
        #[sealed]
        impl Sample for $sample {
            const SAMPLER_TYPE: SamplerType = SamplerType::$ty;

            type Gl = <$sample as Block<Sl>>::Gl;
        }

        #[sealed]
        impl ColorSample for $sample {}
    };
}

impl_color_sample!(F32, Sampler2d);
impl_color_sample!(I32, ISampler2d);
impl_color_sample!(U32, USampler2d);
impl_color_sample!(Vec2, Sampler2d);
impl_color_sample!(IVec2, ISampler2d);
impl_color_sample!(UVec2, USampler2d);
impl_color_sample!(Vec3, Sampler2d);
impl_color_sample!(IVec3, ISampler2d);
impl_color_sample!(UVec3, USampler2d);
impl_color_sample!(Vec4, Sampler2d);
impl_color_sample!(IVec4, ISampler2d);
impl_color_sample!(UVec4, USampler2d);

pub struct Depth;

#[sealed]
impl Sample for Depth {
    const SAMPLER_TYPE: SamplerType = SamplerType::Sampler2dComparison;

    type Gl = f32;
}

/// An object which can be sampled.
#[derive(Debug, Copy, Clone)]
pub struct Sampler2d<S = Vec4> {
    trace: Trace,
    _phantom: PhantomData<S>,
}

impl<S: ColorSample> Object for Sampler2d<S> {
    fn ty() -> Type {
        Type::BuiltIn(BuiltInType::Sampler(SamplerType::Sampler2d))
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
            _phantom: PhantomData,
        }
    }
}

impl<S: ColorSample> Sampler2d<S> {
    pub fn lookup(self, tex_coords: Vec2) -> S {
        // TODO: Convert sample
        built_in_2("texture", self, tex_coords)
    }
}
