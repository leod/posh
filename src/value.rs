pub(crate) mod expr_reg;
mod funcs;
mod primitives;
mod scalar;
mod vec;

use crate::lang::{BuiltInTy, Expr, StructTy, Ty};

pub use funcs::GenValue;
pub use primitives::{and, common_field_base, field, func_call, or, ternary, var};
pub use scalar::{Bool, Scalar, ScalarType, F32, I32, U32};
pub use vec::{vec3, Vec3, Vec4};

pub(crate) use primitives::{binary, builtin1, builtin2, builtin3, builtin4};

use expr_reg::ExprId;

pub trait Type {
    fn ty() -> Ty;
}

pub trait Transparent: Type + IntoValue {
    #[doc(hidden)]
    fn transparent();
}

pub trait BuiltIn: Type {
    fn built_in_ty() -> BuiltInTy;
}

pub trait Struct: Type {
    fn struct_ty() -> StructTy;
}

pub type Val<T> = <T as IntoValue>::Value;

#[derive(Debug, Copy, Clone)]
pub struct Trace {
    expr_id: ExprId,
}

pub trait Value: Copy + Sized {
    type Type: Type;

    fn from_trace(trace: Trace) -> Self;

    fn expr(&self) -> Expr;

    fn from_expr(expr: Expr) -> Self {
        Self::from_trace(Trace::new(expr))
    }

    fn ty(&self) -> Ty {
        Self::Type::ty()
    }

    fn with_trace(&self, trace: Trace) -> Self {
        Self::from_trace(trace)
    }

    fn with_expr(&self, expr: Expr) -> Self {
        Self::from_expr(expr)
    }
}

pub trait BuiltInValue: Value {
    type BuiltInType: BuiltIn;
}

pub trait StructValue: Value {
    type StructType: Struct;

    fn fields(&self) -> Vec<Expr>;
}

pub trait IntoValue {
    type Value: Value;

    fn into_value(self) -> Self::Value;
}

impl<T, V> BuiltInValue for V
where
    T: BuiltIn,
    V: Value<Type = T>,
{
    type BuiltInType = T;
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
