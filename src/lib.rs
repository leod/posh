#![feature(generic_associated_types)]

pub mod lang;
pub mod prelude;
pub mod shader;
pub mod value;

pub use uuid;

pub use prelude::*;

pub use value::{GenValue, IntoValue, Struct, Type, Value};

pub use posh_macros::{posh, Struct};
