use std::rc::Rc;

use crate::dag::{BuiltInType, Expr, SamplerType, Trace, Type};

use super::{primitives::built_in_2, Object, Vec2, Vec4};

/// An object which can be sampled.
#[derive(Debug, Copy, Clone)]
pub struct Sampler2d {
    trace: Trace,
}

impl Object for Sampler2d {
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
        }
    }
}

impl Sampler2d {
    pub fn lookup(self, tex_coords: Vec2) -> Vec4 {
        built_in_2("texture", self, tex_coords)
    }
}
