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
pub use scalar::{Scalar, ScalarType};
pub use trace::Trace;
pub use vec::{vec3, Vec3, Vec4};

pub(crate) use primitives::{binary, builtin1, builtin2, builtin3, builtin4};

pub trait Lift {
    type Type: Copy + Lift<Type = Self::Type>;
}

pub type Po<T> = <T as Lift>::Type;

pub trait IntoPosh: Lift {
    fn into_posh(self) -> Po<Self>;
}

pub trait ValueBase: Copy + Lift<Type = Self> + Sized {
    fn ty() -> Ty;
    fn expr(&self) -> Expr;

    #[doc(hidden)]
    fn from_ident(ident: Ident) -> Self;
}

pub trait Value: ValueBase {
    fn from_trace(trace: Trace) -> Self;

    fn from_expr(expr: Expr) -> Self {
        Self::from_trace(Trace::new(expr))
    }
}

pub trait FuncArg: ValueBase {}

impl<V: Value> FuncArg for V {}

impl<V> IntoPosh for V
where
    V: Lift<Type = Self> + ValueBase,
{
    fn into_posh(self) -> Self {
        self
    }
}
