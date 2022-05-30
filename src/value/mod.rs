mod primitives;
mod scalar;

use crate::{
    expr_reg::{self, ExprId},
    lang::{Expr, Type},
};

pub use primitives::{and, branch, func_call, or, var};
pub use scalar::{Bool, Scalar, ScalarValueType, F32, U32};

pub trait ValueType {
    type Value;

    fn ty() -> Type;
}

pub type Fush<T> = <T as ValueType>::Value;

pub trait Value: Clone + Sized {
    type Type: ValueType;

    fn from_expr_id(expr_id: ExprId) -> Self;
    fn expr_id(&self) -> ExprId;

    fn ty(&self) -> Type {
        Self::Type::ty()
    }

    fn from_expr(expr: Expr) -> Self {
        Self::from_expr_id(expr_reg::put(expr))
    }

    fn expr(&self) -> Expr {
        expr_reg::get(self.expr_id())
    }

    fn map_expr(self, f: impl FnOnce(Expr) -> Expr) -> Self {
        Self::from_expr(f(self.expr()))
    }
}
