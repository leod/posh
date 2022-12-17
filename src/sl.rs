mod program_def;
mod sampler;
mod scalar;
mod tuple;
mod vec;

#[doc(hidden)]
pub mod primitives;

use std::rc::Rc;

use crate::dag::StructTy;

use super::dag::{Expr, Ty};

pub use posh_derive::Value;

pub use {
    program_def::ProgramDef,
    sampler::Sampler2d,
    scalar::{Scalar, F32, I32, U32},
    vec::{Vec2, Vec4},
};

/// An object that can be represented in [`Posh`](crate::Posh).
///
/// The interface of this trait is a private implementation detail.
pub trait Object {
    #[doc(hidden)]
    const TY: Ty;

    #[doc(hidden)]
    fn expr(&self) -> Rc<Expr>;
}

/// An object that can be stored as a value in [`Posh`](crate::Posh).
///
/// The interface of this trait is a private implementation detail.
pub trait Value: Object {
    #[doc(hidden)]
    fn from_expr(expr: Expr) -> Self;
}

pub trait Struct: Value {
    const STRUCT_TY: StructTy;
}

/// A conversion to a [`Value`] in [`Posh`](crate::Posh).
pub trait ToValue {
    type Value: Value;

    fn to_value(self) -> Self::Value;
}

impl<V> ToValue for V
where
    V: Value,
{
    type Value = Self;

    fn to_value(self) -> Self::Value {
        self
    }
}

pub trait Varying: Value {}
