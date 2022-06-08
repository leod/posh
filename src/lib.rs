pub mod lang;
pub mod prelude;
pub mod shader;
pub mod value;

pub use static_assertions;
pub use uuid;

pub use prelude::*;

pub use shader::{
    DescriptorSet, FragIn, FragOut, Fragment, Shader, VertIn, VertOut, Vertex, VertexAttributes,
    VertexOutputs,
};
pub use value::{GenValue, IntoValue, Struct, Transparent, Type, Value};

pub use posh_macros::{posh, Struct, Transparent};

// This is here so that our macros can refer to `posh` even when we use them inside this crate.
extern crate self as posh;
