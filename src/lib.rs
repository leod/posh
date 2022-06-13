pub mod lang;
pub mod prelude;
pub mod shader;
pub mod value;

pub use static_assertions;
pub use uuid;

pub use prelude::*;

pub use shader::{
    FSIn, FSOut, FragmentOut, Resource, Resources, Shader, VSIn, VSOut, Vertex, VertexIn, VertexOut,
};
pub use value::{GenValue, IntoPosh, Sampler2, Value};

pub use posh_macros::{posh, IntoPosh};

// This is here so that our macros can refer to `posh` even when we use them inside this crate.
extern crate self as posh;
