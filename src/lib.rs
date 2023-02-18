mod codegen;
mod compile;
mod dag;
mod interface;
mod program_def;

pub mod gl;
#[macro_use]
pub mod sl;

pub use interface::{
    Block, BlockView, FragmentData, FragmentDataView, Logical, Physical, UniformData,
    UniformDataView, VertexData, VertexDataView,
};

pub use posh_derive::{Block, UniformData, VertexData};

pub use crevice;

/// Utilities.
pub mod util {
    pub use super::{
        compile::{compile_to_program_def, compile_to_program_def_with_consts},
        program_def::{
            ProgramDef, UniformBlockDef, UniformSamplerDef, VertexAttributeDef, VertexDef,
            VertexInputRate,
        },
    };
}

// Only re-exported for `posh-derive`.
// FIXME: Use `crevice`'s re-export.
#[doc(hidden)]
pub use bytemuck;

// Hidden unstable symbols, needed for `posh-derive`.
#[doc(hidden)]
pub mod internal {
    pub use super::{
        dag::{Expr, StructType, Type},
        interface::{UniformDataVisitor, VertexDataField, VertexDataVisitor},
        sl::{
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
