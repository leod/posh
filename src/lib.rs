mod interface;

pub mod gl;
#[macro_use]
pub mod sl;

pub use interface::{
    Block, BlockDom, Fragment, FragmentDom, Gl, Sl, Uniform, UniformDom, UniformNonUnit,
    UniformUnion, Vertex, VertexDom,
};

pub use posh_derive::{Block, Uniform, Vertex};

pub use crevice;
pub use glow;

// Only re-exported for `posh-derive`.
// FIXME: Use `crevice`'s re-export.
#[doc(hidden)]
pub use bytemuck;

// Hidden unstable symbols, needed for `posh-derive`.
#[doc(hidden)]
pub mod internal {
    pub use super::{
        interface::{UniformVisitor, VertexField, VertexVisitor},
        sl::{
            dag::{Expr, StructType, Type},
            primitives::{field, simplify_struct_literal, value_arg},
            unique_struct_type,
        },
    };

    #[doc(hidden)]
    pub fn join_ident_path(lhs: &str, rhs: &str) -> String {
        format!("{lhs}_{rhs}")
    }
}

// Re-export `crate` as `posh` for `posh-derive`.
extern crate self as posh;
