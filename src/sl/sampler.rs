use std::{marker::PhantomData, rc::Rc};

use crate::{
    dag::{BaseType, Expr, Trace, Type},
    Numeric,
};

use super::{primitives::built_in_1, Object, Vec2, Vec4};

/// An object which can be sampled in the shading language.
#[derive(Debug, Copy, Clone)]
pub struct Sampler2d<T: Numeric> {
    trace: Trace,
    _phantom: PhantomData<T>,
}

impl<T: Numeric> Object for Sampler2d<T> {
    fn ty() -> Type {
        Type::Base(BaseType::Sampler2d(T::NUMERIC_TYPE))
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

impl<T: Numeric> Sampler2d<T> {
    pub fn lookup(self, tex_coords: Vec2<f32>) -> Vec4<T> {
        built_in_1("texture", tex_coords)
    }
}
