mod interface;
mod numeric;

#[doc(hidden)]
pub mod dag;

/// The graphics library.
pub mod gl;

/// The shading language.
pub mod sl;

pub use interface::{
    Domain, FragmentDomain, FragmentInterface, ResourceDomain, ResourceInterface, ToPod, Uniform,
    Vertex, VertexDomain, VertexInterface, VertexInterfaceField,
};
pub use numeric::{Numeric, Primitive};

pub use posh_derive::{Uniform, Vertex, VertexInterface};

pub use bytemuck;
pub use crevice;

// Hidden re-exports, needed for `posh-derive`:
#[doc(hidden)]
pub mod derive_internal {
    pub use super::{
        interface::{join_ident_path, VertexAttribute, VertexInSl, VertexInterfaceVisitor},
        sl::primitives,
    };
}

/// The graphics library domain.
///
/// This is the domain in which data for draw calls is specified.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gl;

/// The shading language domain.
///
/// This is the domain in which shaders are defined.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sl;
