mod interface;
mod numeric;

#[doc(hidden)]
pub mod dag;

/// The graphics library.
pub mod gl;

/// The shading language.
pub mod sl;

pub use interface::{
    Domain, Fragment, FragmentDomain, FragmentInterface, Resource, ResourceDomain,
    ResourceInterface, ToPod, Uniform, Vertex, VertexDomain, VertexInterface,
};
pub use numeric::{Numeric, Primitive};

pub use bytemuck;
pub use crevice;

pub use posh_derive::{Uniform, Vertex};

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
