pub mod lang;
pub mod prelude;
pub mod shader;
pub mod value;

pub use uuid;

pub use prelude::*;

pub use shader::{
    shader, FragIn, FragOut, Fragment, ParamSet, ParamSets, Shader, Varying, VertIn, VertOut,
    Vertex, VertexSet,
};
pub use value::{GenValue, IntoValue, Struct, Type, Value};

pub use posh_macros::{posh, Struct};

// This is here so that our macros can refer to `posh` even when we use them inside this crate.
extern crate self as posh;
