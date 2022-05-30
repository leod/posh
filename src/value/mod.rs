pub(crate) mod expr_reg;
mod primitives;
mod scalar;

use crate::lang::{Expr, Type};

pub use primitives::{and, func_call, or, ternary, var};
pub use scalar::{Bool, Scalar, ScalarValueType, F32, U32};

pub(crate) use primitives::binary;

use expr_reg::ExprId;

pub trait ValueType {
    type Value;

    fn ty() -> Type;
}

pub type Fsl<T> = <T as ValueType>::Value;

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

    fn with_expr(&self, expr: Expr) -> Self {
        Self::from_expr(expr)
    }
}

pub trait IntoValue {
    type Value: Value;

    fn into_value(self) -> Self::Value;
}
