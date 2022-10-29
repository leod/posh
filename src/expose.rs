#[macro_use]
mod built_in_value;
pub mod compile;
mod gen_value;
#[cfg(feature = "nalgebra")]
mod nalgebra;
mod primitives;
mod sampler;
mod scalar;
mod trace;
mod tuple;
mod vec;

use std::rc::Rc;

use crate::lang::{Expr, Ty};

pub use built_in_value::BuiltInValue;
pub use gen_value::GenValue;
pub use primitives::{common_field_base, field, func_def_and_call};
pub use sampler::Sampler2;
pub use scalar::{NumType, Scalar, ScalarType};
pub use trace::Trace;
pub use vec::{vec2, vec3, vec4, Vec2, Vec3, Vec4};

pub(crate) use primitives::{binary, built_in1, built_in2, built_in3, built_in4};

/// Exposes a type to Posh through a representative.
///
/// This is the door into the Posh universe. Shaders written in Posh receive inputs as
/// representatives, perform calculations through representatives, and, finally, provide outputs as
/// representatives. User-defined types need to implement `Expose` in order to become accessible in
/// Posh through a representative.
///
/// Internally, representatives track expression trees, making it possible to transpile Posh shaders
/// to other shader languages.
///
/// The recommended way to refer to representatives is [`Posh<T>`].
pub trait Expose {
    /// The type that represents `Self` in Posh.
    type Rep: Rep;
}

/// Maps a type implementing [`Expose`] to its representative in Posh.
///
/// This is the recommended way to refer to Posh's representatives.
///
/// # Examples
/// - `Posh<f32>` is [`Scalar<f32>`]
/// - `Posh<i32>` is [`Scalar<i32>`]
/// - `Posh<bool>` is [`Scalar<bool>`]
/// - `Posh<[f32; 3]>` is [`Vec3<f32>`]
/// - `Posh<nalgebra::Vector3<f32>>` is [`Vec3<f32>`]
pub type Posh<T> = <T as Expose>::Rep;

/// A value-to-value conversion to a representative in Posh.
pub trait IntoPosh: Expose {
    fn into_posh(self) -> Posh<Self>;
}

/// An object which is accessible in Posh.
///
/// This is the supertrait for representatives of types in Posh. Many representatives additionally
/// implement the subtraits [`FuncArg`] and [`Value`], allowing them to be passed to user-defined
/// Posh functions and to be stored in variables in Posh respectively.
///
/// The recommended way to refer to representatives is [`Posh<T>`].
pub trait Rep: Copy + Expose<Rep = Self> {}

/// A representative which can be passed to user-defined Posh functions.
///
/// Implementors of `FuncArg` typically also implement the subtrait [`Value`]. Types which implement
/// `FuncArg` but not `Value` are called *opaque*. Objects of opaque types are not
/// user-constructible. An example is [`Sampler2`].
///
/// For more information on function definitions in Posh, see [`def`](attr.def.html).
pub trait FuncArg: Rep {
    fn ty() -> Ty;
    fn expr(&self) -> Rc<Expr>;

    #[doc(hidden)]
    fn from_var_name(name: &str) -> Self;
}

/// A representative which can be stored in variables in Posh.
pub trait Value: FuncArg {
    #[doc(hidden)]
    fn from_trace(trace: Trace) -> Self;

    #[doc(hidden)]
    fn from_expr(expr: Expr) -> Self {
        Self::from_trace(Trace::new(expr))
    }

    #[doc(hidden)]
    fn must_impl() {}
}

impl<V> IntoPosh for V
where
    V: Expose<Rep = Self>,
{
    fn into_posh(self) -> Self {
        self
    }
}
