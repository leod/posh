mod interface;
mod numeric;

#[doc(hidden)]
pub mod dag;

pub mod gl;
pub mod sl;

pub use interface::{
    Attachment, Attributes, AttributesDomain, Fragment, FragmentDomain, Resource, ResourceDomain,
    Uniform, UniformDomain, Vertex, VertexDomain,
};
pub use numeric::{Numeric, Primitive};

pub use crevice;

pub use posh_derive::Uniform;

/// The graphics library domain.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gl;

/// The shading language domain.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sl;
