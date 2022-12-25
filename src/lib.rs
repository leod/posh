mod interface;
mod numeric;

#[doc(hidden)]
pub mod dag;

pub mod gl;
pub mod sl;

pub use interface::{
    Attachment, Attributes, AttributesDomain, FieldDomain, Fragment, FragmentDomain, Resource,
    ResourceDomain, ToPod, Uniform, Vertex,
};
pub use numeric::{Numeric, Primitive};

pub use bytemuck;
pub use crevice;

pub use posh_derive::{Uniform, Vertex};

/// The graphics library domain.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gl;

/// The shading language domain.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sl;
