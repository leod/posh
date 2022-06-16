pub mod lang;
pub mod shader;
pub mod value;

pub use static_assertions;
pub use uuid;

pub use shader::{
    FSIn, FSOut, FragmentOut, Resource, Resources, Shader, VSIn, VSOut, Vertex, VertexIn, VertexOut,
};
pub use value::{var, vec3, IntoValue, Sampler2, GenValue, Value, Vec3, Vec4, Po};

pub use posh_macros::{posh, IntoValue};

// This is here so that our macros can refer to `posh` even when we use them inside this crate.
extern crate self as posh;
