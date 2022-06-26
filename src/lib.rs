pub mod lang;
pub mod shader;
pub mod value;

pub use static_assertions;
pub use uuid;

pub use shader::{
    FOutputs, FStageArg, FStageRes, Resource, Resources, Shader, VInputs, VOutputs, VStageArg,
    VStageRes, Vertex,
};
pub use value::{var, vec3, GenValue, IntoValue, Lift, Po, Sampler2, Value, Vec3, Vec4};

pub use posh_macros::{posh, IntoValue};

// This is here so that our macros can refer to `posh` even when we use them inside this crate.
extern crate self as posh;
