pub(crate) mod expr_reg;
mod funcs;
mod primitives;
mod sampler;
mod scalar;
mod vec;

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

pub trait Lift {
    type Posh: Copy;
}

pub trait Struct: Type + Lift<Posh = <Self as Struct>::Posh> {
    type Posh: TransparentValue;

    fn struct_ty() -> StructTy;
}

#[derive(Debug, Copy, Clone)]
pub struct Trace {
    expr_id: ExprId,
}

pub trait Value: Copy + Sized {
    type Type: Type + Lift;

    #[doc(hidden)]
    fn from_ident(ident: Ident) -> Self;

    fn expr(&self) -> Expr;

    fn ty(&self) -> Ty {
        Self::Type::ty()
    }
}

pub trait TransparentValue: Value {
    fn from_trace(trace: Trace) -> Self;

    fn from_expr(expr: Expr) -> Self {
        Self::from_trace(Trace::new(expr))
    }

    fn with_trace(&self, trace: Trace) -> Self {
        Self::from_trace(trace)
    }

    fn with_expr(&self, expr: Expr) -> Self {
        Self::from_expr(expr)
    }
}

pub trait FuncArg: Value {}

pub trait IntoPosh: Lift {
    fn into_posh(self) -> Self::Posh;
}

pub type Posh<T> = <T as Lift>::Posh;

impl<V: TransparentValue> FuncArg for V {}

impl<V: Value> Lift for V {
    type Posh = Self;
}

impl<V: Value> IntoPosh for V {
    fn into_posh(self) -> Self {
        self
    }
}

impl Trace {
    pub fn new(expr: Expr) -> Self {
        Self {
            expr_id: expr_reg::put(expr),
        }
    }

    pub fn from_ident<R: Value>(ident: Ident) -> Self {
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
