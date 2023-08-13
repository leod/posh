mod interface;

pub mod gl;
#[macro_use]
pub mod sl;

pub use interface::{
    Block, BlockDom, FsInterface, FsInterfaceDom, UniformInterface, UniformInterfaceDom,
    UniformNonUnit, UniformUnion, VsInterface, VsInterfaceDom,
};

pub use posh_derive::{Block, FsInterface, UniformInterface, VsInterface};

pub use bytemuck;
pub use crevice;
#[cfg(feature = "glam")]
pub use glam;
pub use glow;
#[cfg(feature = "mint")]
pub use mint;

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
