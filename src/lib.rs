mod build;
mod dag;
mod gen;
mod interface;
mod program_def;

pub mod gl;
pub mod sl;

pub use interface::{
    Block, BlockDomain, FragmentDomain, FragmentInterface, Numeric, Primitive, ResourceDomain,
    ResourceInterface, VertexDomain, VertexInterface,
};

pub use posh_derive::{Block, ResourceInterface, VertexInterface};

pub use bytemuck;
pub use crevice;

/// Utilities.
pub mod util {
    pub use super::{
        build::{build_program_def, build_program_def_with_consts},
        program_def::{
            ProgramDef, SamplerDef, UniformDef, VertexAttributeDef, VertexDef, VertexInputRate,
        },
    };
}

/// The graphics library domain.
///
/// This is the domain in which data for draw calls are specified.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gl;

/// The shading language domain.
///
/// This is the domain in which shaders are defined.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sl;

// Hidden unstable symbols, needed for `posh-derive`:
#[doc(hidden)]
pub mod internal {
    pub use super::{
        dag::{BaseType, Expr, StructType, Type},
        interface::{ResourceInterfaceVisitor, VertexInterfaceField, VertexInterfaceVisitor},
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
