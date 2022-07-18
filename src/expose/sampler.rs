use crate::lang::{BuiltInTy, Expr, Ident, Ty};

use super::{built_in2, Expose, FuncArg, Representative, Trace, Vec2, Vec4};

/// Representative for samplers.
#[derive(Debug, Copy, Clone)]
pub struct Sampler2(Trace);

impl Expose for Sampler2 {
    type Rep = Self;
}

impl Representative for Sampler2 {}

impl FuncArg for Sampler2 {
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

impl Sampler2 {
    pub fn load(self, tex_coord: Vec2<f32>) -> Vec4<f32> {
        built_in2("texture", self, tex_coord)
    }
}
