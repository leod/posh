pub mod lang;
pub mod shader;
pub mod value;

pub use static_assertions;
pub use uuid;

pub use shader::{
    FOutputs, FStageIn, FStageOut, Resource, Resources, Shader, VInputs, VOutputs, VStageIn,
    VStageOut, Vertex,
};
pub use value::{
    var, vec3, ConstructibleVal, FuncArgVal, GenVal, IntoVal, Sampler2, Type, TypedVal, Val, Value,
    Vec3, Vec4,
};

pub use posh_macros::{def, IntoVal};

// This was here so that our macros could refer to `posh` even when we use them inside this crate.
//extern crate self as posh;
