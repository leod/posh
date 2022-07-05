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

/// Exposes a Rust type to Posh through a representative type.
pub trait Expose {
    /// The type that represents `Self` in Posh.
    type Rep: Representative;
}

/// Maps a type implementing [`Expose`] to its representative in Posh.
///
/// # Examples
/// - `Rep<f32>` is [`Scalar<f32>`]
/// - `Rep<i32>` is [`Scalar<i32>`]
/// - `Rep<bool>` is [`Scalar<bool>`]
/// - `Rep<[f32; 3]>` is [`Vec3<f32>`]
pub type Rep<T> = <T as Expose>::Rep;

/// A value-to-value conversion to a representative in Posh.
pub trait IntoRep: Expose {
    fn into_rep(self) -> Rep<Self>;
}

/// An object which is accessible in Posh.
pub trait Representative: Copy + Expose<Rep = Self> + Sized {}

/// A representative which has a [`Ty`] in Posh and can be mapped to an [`Expr`].
pub trait MapToExpr: Representative {
    fn ty() -> Ty;
    fn expr(&self) -> Expr;

    #[doc(hidden)]
    fn from_ident(ident: Ident) -> Self;
}

/// A representative which can be freely used as a value in Posh.
pub trait Value: MapToExpr {
    #[doc(hidden)]
    fn from_trace(trace: Trace) -> Self;

    #[doc(hidden)]
    fn from_expr(expr: Expr) -> Self {
        Self::from_trace(Trace::new(expr))
    }
}

/// A representative which can be passed to user-defined Posh functions.
pub trait FuncArg: MapToExpr {}

impl<V: Value> FuncArg for V {}

impl<V> IntoRep for V
where
    V: Expose<Rep = Self> + MapToExpr,
{
    fn into_rep(self) -> Self {
        self
    }
}
