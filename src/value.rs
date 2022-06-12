pub(crate) mod expr_reg;
mod funcs;
mod primitives;
mod sampler;
mod scalar;
mod vec;

use std::marker::PhantomData;

use crate::lang::{Expr, Ident, StructTy, Ty, VarExpr};

pub use funcs::GenValue;
pub use primitives::{common_field_base, field, func_def_and_call, var};
pub use sampler::Sampler2;
pub use scalar::{Bool, Scalar, ScalarType, F32, I32, U32};
pub use vec::{vec3, Vec3, Vec4};

pub(crate) use primitives::{binary, builtin1, builtin2, builtin3, builtin4};

use expr_reg::ExprId;

pub trait Type {
    fn ty() -> Ty;
}

pub trait Lift<'a> {
    type Posh: Copy;
}

pub trait Struct: Type + for<'a> Lift<'a, Posh = <Self as Struct>::Posh> {
    type Posh: for<'a> TransparentValue<'a>;

    fn struct_ty() -> StructTy;
}

#[derive(Debug, Copy, Clone)]
pub struct Trace<'a> {
    expr_id: ExprId,
    _phantom: PhantomData<&'a ()>,
}

pub trait Value<'a>: Copy + Sized {
    type Type: Type + Lift<'a>;

    #[doc(hidden)]
    fn from_ident(ident: Ident) -> Self;

    fn expr(&self) -> Expr;

    fn ty(&self) -> Ty {
        Self::Type::ty()
    }
}

pub trait TransparentValue<'a>: Value<'a> {
    fn from_trace(trace: Trace<'a>) -> Self;

    fn from_expr(expr: Expr) -> Self {
        Self::from_trace(Trace::new(expr))
    }
}

pub trait FuncArg<'a>: Value<'a> {}

pub trait IntoPosh<'a>: Lift<'a> {
    fn into_posh(self) -> Self::Posh;
}

pub type Posh<'a, T> = <T as Lift<'a>>::Posh;

impl<'a, V: TransparentValue<'a>> FuncArg<'a> for V {}

impl<'a, V: Value<'a>> Lift<'a> for V {
    type Posh = Self;
}

impl<'a, V: Value<'a>> IntoPosh<'a> for V {
    fn into_posh(self) -> Self {
        self
    }
}

impl<'a> Trace<'a> {
    pub fn new(expr: Expr) -> Self {
        Self {
            expr_id: expr_reg::put(expr),
            _phantom: PhantomData,
        }
    }

    pub fn from_ident<R: Value<'a>>(ident: Ident) -> Self {
        Self::new(Expr::Var(VarExpr {
            ident,
            ty: <R::Type as Type>::ty(),
            init: None,
        }))
    }

    pub fn expr(&self) -> Expr {
        expr_reg::get(self.expr_id)
    }
}
