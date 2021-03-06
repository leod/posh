#[doc(hidden)]
#[macro_use]
pub mod expose;
pub mod lang;
pub mod shader;

#[doc(hidden)]
pub use static_assertions;
#[doc(hidden)]
pub use uuid;

pub use expose::{
    var, vec2, vec3, vec4, BuiltInValue, Expose, FuncArg, GenValue, IntoRep, NumType, Rep,
    Representative, Sampler2, Scalar, ScalarType, Value, Vec2, Vec3, Vec4,
};

pub use posh_macros::{def, Expose};

// This was here so that our macros could refer to `posh` even when we use them inside this crate.
//extern crate self as posh;
