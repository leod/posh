mod interface;

pub mod gl;
#[macro_use]
pub mod sl;

pub use interface::{
    Block, BlockView, FragmentData, FragmentDataView, Logical, Physical, UniformData,
    UniformDataNonUnit, UniformDataUnion, UniformDataView, VertexData, VertexDataView,
};

pub use posh_derive::{Block, UniformData, VertexData};

pub use crevice;

// Only re-exported for `posh-derive`.
// FIXME: Use `crevice`'s re-export.
#[doc(hidden)]
pub use bytemuck;

// Hidden unstable symbols, needed for `posh-derive`.
#[doc(hidden)]
pub mod internal {
    pub use super::{
        interface::{UniformDataVisitor, VertexDataField, VertexDataVisitor},
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
