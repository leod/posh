#[macro_use]
mod built_in_value;
mod expr_reg;
mod gen_value;
#[cfg(feature = "nalgebra")]
mod nalgebra;
mod primitives;
mod sampler;
mod scalar;
mod trace;
mod tuple;
mod vec;

use crate::lang::{Expr, Ident, Ty};

pub use built_in_value::BuiltInValue;
pub use gen_value::GenValue;
pub use primitives::{common_field_base, field, func_def_and_call, var};
pub use sampler::Sampler2;
pub use scalar::{Scalar, ScalarType};
pub use trace::Trace;
pub use vec::{vec2, vec3, vec4, Vec2, Vec3, Vec4};

pub(crate) use primitives::{binary, built_in1, built_in2, built_in3, built_in4};
pub(crate) use scalar::NumericType;

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
pub trait Representative: Copy + Expose<Rep = Self> {}

/// A representative which can be passed to user-defined Posh functions.
///
/// For more information on function definitions in Posh, see [`def`](attr.def.html).
///
/// Almost all implementors of `FuncArg` also implement the subtrait [`Value`]. Opaque types are an
/// exception to this. For example, the opaque type [`Sampler2`] can be passed to functions in Posh,
/// but it can *not* be stored in variables in Posh.
pub trait FuncArg: Representative {
    fn ty() -> Ty;
    fn expr(&self) -> Expr;

    /// FIXME
    #[doc(hidden)]
    fn from_ident(ident: Ident) -> Self;
}

/// A representative which can be stored in variables in Posh.
pub trait Value: FuncArg {
    #[doc(hidden)]
    fn from_trace(trace: Trace) -> Self;

    #[doc(hidden)]
    fn from_expr(expr: Expr) -> Self {
        Self::from_trace(Trace::new(expr))
    }
}

impl<V> IntoRep for V
where
    V: Expose<Rep = Self>,
{
    fn into_rep(self) -> Self {
        self
    }
}
