mod interface;
mod numeric;

#[doc(hidden)]
pub mod dag;

pub mod gl;
pub mod sl;

pub use interface::{
    Attachment, Attributes, AttributesDomain, Fragment, FragmentDomain, Resource, ResourceDomain,
    Uniform, UniformDomain, UniformField, Vertex, VertexDomain, VertexField,
};
pub use numeric::{Numeric, Primitive};

/// The graphics library domain.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gl;

/// The shading language domain.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sl;
