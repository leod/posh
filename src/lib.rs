#[macro_use]
#[doc(hidden)]
pub mod expose;
pub mod lang;
pub mod scope;
pub mod shader;

#[doc(hidden)]
pub use static_assertions;

pub use expose::{
    vec2, vec3, vec4, BuiltInValue, Expose, FuncArg, GenValue, IntoRep, NumType, Rep,
    Representative, Sampler2, Scalar, ScalarType, Value, Vec2, Vec3, Vec4,
};

pub use posh_macros::{def, rep, Expose};
