mod program_def;
mod sampler;
mod scalar;
mod tuple;
mod vec;

#[doc(hidden)]
pub mod primitives;

use std::rc::Rc;

use crate::dag::StructType;

use super::dag::{Expr, Type};

pub use posh_derive::{ToValue, Value};

pub use {
    program_def::ProgramDef,
    sampler::Sampler2d,
    scalar::{Bool, Scalar, F32, I32, U32},
    vec::{Vec2, Vec4},
};

/// An object that can be represented in the shading language domain
/// [`Sl`](crate::Sl).
///
/// The interface of this trait is a private implementation detail.
pub trait Object {
    #[doc(hidden)]
    const TYPE: Type;

    #[doc(hidden)]
    fn expr(&self) -> Rc<Expr>;
}

/// An object that can be stored as a value in the shading language domain
/// [`Sl`](crate::Sl).
///
/// The interface of this trait is a private implementation detail.
pub trait Value: Object {
    #[doc(hidden)]
    fn from_expr(expr: Expr) -> Self;
}

/// A [`Value`] that has a struct type in the shading language domain
/// [`Sl`](crate::Sl).
pub trait Struct: Value {
    #[doc(hidden)]
    const STRUCT_TYPE: StructType;
}

/// A conversion to a [`Value`] in the shading language domain
/// [`Sl`](crate::Sl).
pub trait ToValue: Copy {
    type Output: Value;

    fn to_value(self) -> Self::Output;
}

/// Data passed from vertex shader to fragment shader.
pub trait Varying: Value {}
