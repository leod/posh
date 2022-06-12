use crate::{
    lang::{BuiltInTy, Expr, Ident, Ty},
    Type, Value, Vec3, Vec4,
};

use super::{builtin2, FuncArg, Trace};

#[derive(Debug, Copy, Clone)]
pub struct Sampler2<'a>(Trace<'a>);

impl<'a> Type for Sampler2<'a> {
    fn ty() -> Ty {
        Ty::BuiltIn(BuiltInTy::Sampler2)
    }
}

impl<'a> Value<'a> for Sampler2<'a> {
    type Type = Self;

    fn from_ident(ident: Ident) -> Self {
        Sampler2(Trace::from_ident::<Self>(ident))
    }

    fn expr(&self) -> Expr {
        self.0.expr()
    }
}

impl<'a> FuncArg<'a> for Sampler2<'a> {}

impl<'a> Sampler2<'a> {
    pub fn load(self, tex_coord: Vec3<'a, f32>) -> Vec4<'a, f32> {
        builtin2("texture", self, tex_coord)
    }
}
