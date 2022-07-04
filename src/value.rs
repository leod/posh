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

pub trait Lift {
    type Value: ValueBase;
}

pub type Val<T> = <T as Lift>::Value;

pub trait IntoVal: Lift {
    fn into_val(self) -> Val<Self>;
}

pub trait ValueBase: Copy + Lift<Value = Self> + Sized {}

pub trait Value: ValueBase {
    fn ty() -> Ty;
    fn expr(&self) -> Expr;

    #[doc(hidden)]
    fn from_ident(ident: Ident) -> Self;
}

pub trait Constructible: Value {
    fn from_trace(trace: Trace) -> Self;

    fn from_expr(expr: Expr) -> Self {
        Self::from_trace(Trace::new(expr))
    }
}

pub trait FuncArg: Value {}

impl<V: Constructible> FuncArg for V {}

impl<V> IntoVal for V
where
    V: Lift<Value = Self> + Value,
{
    fn into_val(self) -> Self {
        self
    }
}
