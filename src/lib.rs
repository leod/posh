pub mod lang;
pub mod shader;
#[doc(hidden)]
pub mod value;

#[doc(hidden)]
pub use static_assertions;
#[doc(hidden)]
pub use uuid;

pub use value::{
    var, vec3, Constructible, FuncArg, GenVal, IntoVal, Lift, Sampler2, Val, Value, ValueBase,
    Vec3, Vec4,
};

pub use posh_macros::{def, Lift};

// This was here so that our macros could refer to `posh` even when we use them inside this crate.
//extern crate self as posh;
