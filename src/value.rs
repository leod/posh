pub(crate) mod expr_reg;
mod gen_val;
mod primitives;
mod sampler;
mod scalar;
mod trace;
mod tuple;
mod vec;

use crate::lang::{Expr, Ident, Ty};

pub use gen_val::GenVal;
pub use primitives::{common_field_base, field, func_def_and_call, var};
pub use sampler::Sampler2;
pub use scalar::{Scalar, ScalarType};
pub use trace::Trace;
pub use vec::{vec3, Vec3, Vec4};

pub(crate) use primitives::{binary, builtin1, builtin2, builtin3, builtin4};

pub trait Type {
    type Val: Val;
}

pub type Value<T> = <T as Type>::Val;

pub trait IntoVal: Type {
    fn into_val(self) -> Value<Self>;
}

pub trait Val: Copy + Type<Val = Self> + Sized {}

pub trait TypedVal: Val {
    fn ty() -> Ty;
    fn expr(&self) -> Expr;

    #[doc(hidden)]
    fn from_ident(ident: Ident) -> Self;
}

pub trait ConstructibleVal: TypedVal {
    fn from_trace(trace: Trace) -> Self;

    fn from_expr(expr: Expr) -> Self {
        Self::from_trace(Trace::new(expr))
    }
}

pub trait FuncArgVal: TypedVal {}

impl<V: ConstructibleVal> FuncArgVal for V {}

impl<V> IntoVal for V
where
    V: Type<Val = Self> + TypedVal,
{
    fn into_val(self) -> Self {
        self
    }
}
