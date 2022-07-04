use crate::lang::{BuiltInTy, Expr, Ident, Ty};

use super::{builtin2, FuncArg, Lift, Trace, Value, ValueBase, Vec3, Vec4};

#[derive(Debug, Copy, Clone)]
pub struct Sampler2(Trace);

impl Lift for Sampler2 {
    type Value = Self;
}

impl ValueBase for Sampler2 {}

impl Value for Sampler2 {
    fn from_ident(ident: Ident) -> Self {
        Sampler2(Trace::from_ident::<Self>(ident))
    }

    fn expr(&self) -> Expr {
        self.0.expr()
    }

    fn ty() -> Ty {
        Ty::BuiltIn(BuiltInTy::Sampler2)
    }
}

impl FuncArg for Sampler2 {}

impl Sampler2 {
    pub fn load(self, tex_coord: Vec3<f32>) -> Vec4<f32> {
        builtin2("texture", self, tex_coord)
    }
}
