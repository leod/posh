pub(crate) mod expr_reg;
mod funcs;
mod primitives;
mod sampler;
mod scalar;
mod trace;
mod tuple;
mod vec;

use crate::lang::{Expr, Ident, Ty};

pub use funcs::GenValue;
pub use primitives::{common_field_base, field, func_def_and_call, var};
pub use sampler::Sampler2;
pub use scalar::{Bool, Scalar, ScalarType, F32, I32, U32};
pub use trace::Trace;
pub use vec::{vec3, Vec3, Vec4};

pub(crate) use primitives::{binary, builtin1, builtin2, builtin3, builtin4};

pub trait Lift {
    type Posh: Copy + Lift<Posh = Self::Posh>;
}

pub trait IntoPosh: Lift {
    fn into_posh(self) -> Self::Posh;
}

pub trait Value: Copy + Lift<Posh = Self> + Sized {
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

pub type Posh<T> = <T as Lift>::Posh;

impl<V: Constructible> FuncArg for V {}

impl<V> IntoPosh for V
where
    V: Lift<Posh = Self> + Value,
{
    fn into_posh(self) -> Self {
        self
    }
}
