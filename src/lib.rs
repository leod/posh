pub mod lang;
pub mod shader;
#[doc(hidden)]
pub mod value;

#[doc(hidden)]
pub use static_assertions;
#[doc(hidden)]
pub use uuid;

pub use value::{
    var, vec3, ConstructibleVal, FuncArgVal, GenVal, IntoVal, Sampler2, Type, TypedVal, Val, Value,
    Vec3, Vec4,
};

pub use posh_macros::{def, IntoVal};

// This was here so that our macros could refer to `posh` even when we use them inside this crate.
//extern crate self as posh;
