use bytemuck::Pod;
use crevice::std140::AsStd140;
use sealed::sealed;

mod gl;
mod sl;

use crate::{
    dag::Type,
    sl::{Bool, Scalar, ToValue, Value, Vec2, F32, I32, U32},
    Gl, Numeric, Primitive, Sl,
};

/// Provides types for declaring fields in a [`Uniform`] or a [`Vertex`].
#[sealed]
pub trait Domain: Copy {
    /// A scalar value.
    type Scalar<T: Primitive>: Uniform<Self> + Vertex<Self> + ToValue<Output = Scalar<T>>;

    /// A two-dimensional vector.
    type Vec2<T: Primitive>: Uniform<Self> + Vertex<Self> + ToValue<Output = Vec2<T>>;

    /// A boolean value.
    ///
    /// Shorthand for [`Self::Scalar<bool>`].
    type Bool: Uniform<Self> + Vertex<Self> + ToValue<Output = Bool>;

    /// A floating-point value.
    ///
    /// Shorthand for [`Self::Scalar<f32>`].
    type F32: Uniform<Self> + Vertex<Self> + ToValue<Output = F32>;

    /// A signed integer value.
    ///
    /// Shorthand for [`Self::Scalar<i32>`].
    type I32: Uniform<Self> + Vertex<Self> + ToValue<Output = I32>;

    /// An unsigned integer value.
    ///
    /// Shorthand for [`Self::Scalar<u32>`].
    type U32: Uniform<Self> + Vertex<Self> + ToValue<Output = U32>;
}

// Uniform interface

/// Uniform types.
pub trait Uniform<D: Domain>: ToValue {
    type InGl: Uniform<Gl> + AsStd140 + ToValue<Output = Self::InSl>;
    type InSl: Uniform<Sl> + Value + ToValue<Output = Self::InSl>;
}

// Vertex interface

/// A conversion to a type that implements [`Pod`].
pub trait ToPod: Copy {
    type Output: Pod;

    fn to_pod(self) -> Self::Output;
}

#[doc(hidden)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VertexAttribute {
    pub name: String,
    pub ty: Type,
    pub offset: usize,
}

/// Vertex types.
pub trait Vertex<D: Domain>: ToValue {
    type InGl: Vertex<Gl> + ToPod + ToValue<Output = Self::InSl>;
    type InSl: Vertex<Sl> + Value + ToValue<Output = Self::InSl>;

    #[doc(hidden)]
    fn attributes(path: &mut Vec<&'static str>) -> Vec<VertexAttribute>;
}

#[doc(hidden)]
pub trait VertexInterfaceVisitor<D: VertexDomain> {
    fn accept<V: Vertex<D>>(&mut self, path: &[&str], vertex: &D::Vertex<V>);
}

/// Types that declare a shader input vertex interface.
pub trait VertexInterface<D: VertexDomain> {
    type InGl: VertexInterface<Gl>;
    type InSl: VertexInterface<Sl>;

    fn visit(&self, visitor: &mut impl VertexInterfaceVisitor<D>);
}

/// Provides types for declaring fields in a [`VertexInterface`].
#[sealed]
pub trait VertexDomain: Domain {
    type Vertex<V: Vertex<Self>>: VertexInterface<Self>;
}

// Resource interface

/// Types that declare a shader input resource interface.
pub trait ResourceInterface<D: ResourceDomain> {
    type InGl: ResourceInterface<Gl>;
    type InSl: ResourceInterface<Sl>;
}

/// Provides types for declaring fields in a [`ResourceInterface`].
#[sealed]
pub trait ResourceDomain: Domain {
    type Sampler2d<T: Numeric>: ResourceInterface<Self>;

    type Uniform<U: Uniform<Self>>: ResourceInterface<Self>;
}

// Fragment interface

/// Types that declare a shader output fragment interface.
pub trait FragmentInterface<D: FragmentDomain> {
    type InGl: FragmentInterface<Gl>;
    type InSl: FragmentInterface<Sl>;
}

/// Provides types for declaring fields in a [`FragmentInterface`].
#[sealed]
pub trait FragmentDomain: Sized {
    type Attachment2d: FragmentInterface<Self>;
}
