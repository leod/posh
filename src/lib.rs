#[doc(hidden)]
pub mod expose;
pub mod lang;
pub mod shader;

#[doc(hidden)]
pub use static_assertions;
#[doc(hidden)]
pub use uuid;

pub use expose::{
    var, vec3, BuiltInValue, Expose, FuncArg, GenValue, IntoRep, MapToExpr, Rep, Representative,
    Sampler2, Scalar, Value, Vec3, Vec4,
};

pub use posh_macros::{def, Expose};

// This was here so that our macros could refer to `posh` even when we use them inside this crate.
//extern crate self as posh;
