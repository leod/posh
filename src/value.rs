pub(crate) mod expr_reg;
mod funcs;
mod primitives;
mod scalar;
mod vec;

use crate::lang::{Expr, Type};

pub use funcs::GenValue;
pub use primitives::{and, func_call, or, ternary, var};
pub use scalar::{Bool, Scalar, ScalarValueType, F32, I32, U32};
pub use vec::{vec3, Vec3, Vec4};

pub(crate) use primitives::{binary, builtin1, builtin2, builtin3};

use expr_reg::ExprId;

pub trait ValueType {
    type Value;

    fn ty() -> Type;
}

pub type Posh<T> = <T as ValueType>::Value;

#[derive(Debug, Copy, Clone)]
pub struct Trace {
    expr_id: ExprId,
}

pub trait Value: Clone + Sized {
    type Type: ValueType;

    fn from_trace(trace: Trace) -> Self;
    fn trace(&self) -> Trace;

    fn from_expr(expr: Expr) -> Self {
        Self::from_trace(Trace::new(expr))
    }

    fn ty(&self) -> Type {
        Self::Type::ty()
    }

    fn expr(&self) -> Expr {
        self.trace().expr()
    }

    fn with_trace(&self, trace: Trace) -> Self {
        Self::from_trace(trace)
    }

    fn with_expr(&self, expr: Expr) -> Self {
        Self::from_expr(expr)
    }
}

pub trait IntoValue {
    type Value: Value;

    fn into_value(self) -> Self::Value;
}

impl<V: Value> IntoValue for V {
    type Value = Self;

    fn into_value(self) -> Self::Value {
        self
    }
}

impl Trace {
    pub fn new(expr: Expr) -> Self {
        Self {
            expr_id: expr_reg::put(expr),
        }
    }

    pub fn expr(&self) -> Expr {
        expr_reg::get(self.expr_id)
    }
}
