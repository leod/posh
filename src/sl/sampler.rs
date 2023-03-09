use std::{marker::PhantomData, rc::Rc};

use sealed::sealed;

use super::{
    dag::{BuiltInType, Expr, SamplerType, Trace, Type},
    primitives::built_in_2,
    IVec2, IVec3, IVec4, Object, UVec2, UVec3, UVec4, Value, Vec2, Vec3, Vec4, F32, I32, U32,
};

#[sealed]
pub trait Sample: Value {
    const SAMPLER_TYPE: SamplerType;
}

macro_rules! impl_sample {
    ($sample:ident, $ty:ident) => {
        #[sealed]
        impl Sample for $sample {
            const SAMPLER_TYPE: SamplerType = SamplerType::$ty;
        }
    };
}

impl_sample!(F32, Sampler2d);
impl_sample!(I32, ISampler2d);
impl_sample!(U32, USampler2d);
impl_sample!(Vec2, Sampler2d);
impl_sample!(IVec2, ISampler2d);
impl_sample!(UVec2, USampler2d);
impl_sample!(Vec3, Sampler2d);
impl_sample!(IVec3, ISampler2d);
impl_sample!(UVec3, USampler2d);
impl_sample!(Vec4, Sampler2d);
impl_sample!(IVec4, ISampler2d);
impl_sample!(UVec4, USampler2d);

/// An object which can be sampled.
#[derive(Debug, Copy, Clone)]
pub struct Sampler2d<S = Vec4> {
    trace: Trace,
    _phantom: PhantomData<S>,
}

impl<S: Sample> Object for Sampler2d<S> {
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

impl<S: Sample> Sampler2d<S> {
    pub fn lookup(self, tex_coords: Vec2) -> S {
        // TODO: Convert sample
        built_in_2("texture", self, tex_coords)
    }
}
