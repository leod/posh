mod interface;

/// Imagine macro parameters, but more like those Russian dolls.
///
/// Calls m!(A, B, C), m!(A, B), m!(B), and m!() for i.e. (m, A, B, C)
/// where m is any macro, for any number of parameters.
///
/// Copied from Ralith's hecs crate.
macro_rules! smaller_tuples_too {
    ($m: ident, $ty: ident) => {
        $m!{}
        $m!{$ty}
    };
    ($m: ident, $ty: ident, $($tt: ident),*) => {
        smaller_tuples_too!{$m, $($tt),*}
        $m!{$ty, $($tt),*}
    };
}

pub mod gl;
#[macro_use]
pub mod sl;

pub use interface::{
    Block, BlockDom, FsInterface, FsDom, Uniform, UniformDom, UniformNonUnit,
    UniformUnion, VsDom, VsInterface,
};

pub use posh_derive::{Block, FsInterface, Uniform, VsInterface};

pub use bytemuck;
pub use crevice;
pub use glow;

/// The graphics library's view of shader inputs and outputs.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Gl;

/// The shading language's view of shader inputs and outputs.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Sl;

/// A conversion to a [`Value`] in the shading language.
///
/// This is useful for converting literals to the shading language.
pub trait ToSl: Copy {
    type Output: sl::Value;

    fn to_sl(self) -> Self::Output;
}

// Hidden unstable symbols, needed for `posh-derive`.
#[doc(hidden)]
pub mod internal {
    pub use super::{
        interface::{FragmentVisitor, UniformVisitor, VertexField, VertexVisitor},
        sl::{
            dag::{Expr, StructType, Type},
            primitives::{field, simplify_struct_literal, value_arg},
            unique_struct_type,
        },
    };

    #[doc(hidden)]
    pub fn join_ident_path(lhs: &str, rhs: &str) -> String {
        // FIXME: There is probably a chance that this produces duplicate
        // symbol names.
        format!("{lhs}_{rhs}")
    }
}

// Re-export `crate` as `posh` for `posh-derive`.
extern crate self as posh;
