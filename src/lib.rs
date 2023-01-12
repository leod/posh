mod compile;
mod dag;
mod gen;
mod interface;
mod numeric;

/// The graphics library.
pub mod gl;

/// The shading language.
pub mod sl;

/// A compiled, type-erased program.
pub mod program_def;

pub use compile::compile;
pub use interface::{
    Domain, FragmentDomain, FragmentInterface, ResourceDomain, ResourceInterface, ToPod, Uniform,
    Vertex, VertexDomain, VertexInterface, VertexInterfaceField,
};
pub use numeric::{Numeric, Primitive};

pub use posh_derive::{ResourceInterface, Uniform, Vertex, VertexInterface};

pub use bytemuck;
pub use crevice;

// Hidden re-exports, needed for `posh-derive`:
#[doc(hidden)]
pub mod internal {
    pub use super::{
        dag::{BaseType, Expr, StructType, Type},
        interface::{join_ident_path, ResourceInterfaceVisitor, VertexInterfaceVisitor},
        sl::{
            primitives::{field, simplify_struct_literal, value_arg},
            unique_struct_type,
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
