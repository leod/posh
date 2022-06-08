use crate::{
    lang::{BuiltInTy, BuiltInVarExpr, Expr, Ty},
    Type, Value, Vec3, Vec4,
};

use super::{builtin2, BuiltIn, FuncArg, Trace};

#[derive(Debug, Copy, Clone)]
pub struct Sampler2d(Trace);

impl Type for Sampler2d {
    fn ty() -> Ty {
        Ty::BuiltIn(Self::built_in_ty())
    }
}

impl Value for Sampler2d {
    type Type = Self;

    fn from_trace(trace: Trace) -> Self {
        Sampler2d(trace)
    }

    fn expr(&self) -> Expr {
        self.0.expr()
    }
}

impl FuncArg for Sampler2d {}

impl BuiltIn for Sampler2d {
    fn built_in_ty() -> BuiltInTy {
        BuiltInTy::Sampler2d
    }
}

impl Sampler2d {
    // FIXME: This is just for testing
    pub fn func_arg(name: &'static str) -> Self {
        Self::from_expr(Expr::BuiltInVar(BuiltInVarExpr {
            name: name.to_string(),
            ty: Self::built_in_ty(),
        }))
    }

    pub fn load(self, tex_coord: Vec3<f32>) -> Vec4<f32> {
        builtin2("texture", self, tex_coord)
    }
}
