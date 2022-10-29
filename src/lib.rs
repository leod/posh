#[macro_use]
#[doc(hidden)]
pub mod expose;
pub mod lang;
pub mod shader;
pub mod var_form;

#[doc(hidden)]
pub use static_assertions;

pub use expose::{
    vec2, vec3, vec4, BuiltInValue, Expose, FuncArg, GenValue, IntoPosh, NumType, Posh, Rep,
    Sampler2, Scalar, ScalarType, Value, Vec2, Vec3, Vec4,
};

pub use posh_macros::{def, rep, Expose};
