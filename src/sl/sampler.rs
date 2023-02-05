use std::{marker::PhantomData, rc::Rc};

use sealed::sealed;

use crate::{
    dag::{BaseType, Expr, SamplerType, Trace, Type},
    Numeric,
};

use super::{
    primitives::{built_in_2, cast},
    Object, Scalar, Value, Vec2, Vec3, Vec4,
};

/// A value returned by a sampler.
#[sealed]
pub trait Sample: Value {
    const DIMENSION: usize;

    /// Type of a single component of a sample.
    type Component: Numeric;
}

#[sealed]
impl<T: Numeric> Sample for Scalar<T> {
    const DIMENSION: usize = 1;
    type Component = T;
}

#[sealed]
impl<T: Numeric> Sample for Vec2<T> {
    const DIMENSION: usize = 2;
    type Component = T;
}

#[sealed]
impl<T: Numeric> Sample for Vec3<T> {
    const DIMENSION: usize = 3;
    type Component = T;
}

#[sealed]
impl<T: Numeric> Sample for Vec4<T> {
    const DIMENSION: usize = 4;
    type Component = T;
}

/// An object which can be sampled.
#[derive(Debug, Copy, Clone)]
pub struct Sampler2d<S> {
    trace: Trace,
    _phantom: PhantomData<S>,
}

impl<S: Sample> Object for Sampler2d<S> {
    fn ty() -> Type {
        Type::Base(BaseType::Sampler(SamplerType::Sampler2d {
            dimension: S::DIMENSION,
            ty: <S::Component as Numeric>::NUMERIC_TYPE,
        }))
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
    pub fn lookup(self, tex_coords: Vec2<f32>) -> S {
        let sample: Vec4<S::Component> = built_in_2("texture", self, tex_coords);

        cast(sample)
    }
}
