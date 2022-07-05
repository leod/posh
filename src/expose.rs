pub(crate) mod expr_reg;
mod gen_val;
mod primitives;
mod sampler;
mod scalar;
mod trace;
mod tuple;
mod vec;

use crate::lang::{Expr, Ident, Ty};

pub use gen_val::GenValue;
pub use primitives::{common_field_base, field, func_def_and_call, var};
pub use sampler::Sampler2;
pub use scalar::{Scalar, ScalarType};
pub use trace::Trace;
pub use vec::{vec3, Vec3, Vec4};

pub(crate) use primitives::{binary, builtin1, builtin2, builtin3, builtin4};

/// The `Expose` trait exposes Rust types to Posh.
pub trait Expose {
    /// The representant for `Self` in Posh.
    type Rep: Representant;
}

/// Maps a type `T` implementing [`Expose`] to its representant type `Rep<T>`.
pub type Rep<T> = <T as Expose>::Rep;

/// A value-to-value conversion from `T` to [`Rep<T>`].
pub trait IntoRep: Expose {
    fn into_rep(self) -> Rep<Self>;
}

/// An object which is accessible in Posh.
pub trait Representant: Copy + Expose<Rep = Self> + Sized {}

/// A [`Representant`] which can be accessed as a value.
pub trait ValueBase: Representant {
    fn ty() -> Ty;
    fn expr(&self) -> Expr;

    #[doc(hidden)]
    fn from_ident(ident: Ident) -> Self;
}

/// A [`ValueBase`] which are user-constructible and can be stored in variables.
pub trait Value: ValueBase {
    fn from_trace(trace: Trace) -> Self;

    fn from_expr(expr: Expr) -> Self {
        Self::from_trace(Trace::new(expr))
    }
}

/// [`Value`]s which can be passed to functions.
pub trait FuncArg: ValueBase {}

impl<V: Value> FuncArg for V {}

impl<V> IntoRep for V
where
    V: Expose<Rep = Self> + ValueBase,
{
    fn into_rep(self) -> Self {
        self
    }
}
