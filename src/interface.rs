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

#[doc(hidden)]
pub fn join_ident_path(lhs: &str, rhs: &str) -> String {
    format!("{lhs}_{rhs}")
}

// Uniform interface

/// Uniform types.
pub trait Uniform<D: Domain>: ToValue {
    type InGl: Uniform<Gl> + AsStd140 + ToValue<Output = Self::InSl>;
    type InSl: Uniform<Sl> + Value + ToValue<Output = Self::InSl>;

    #[doc(hidden)]
    fn shader_input(path: &str) -> Self;
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
    fn attributes(path: &str) -> Vec<VertexAttribute>;

    #[doc(hidden)]
    fn shader_input(path: &str) -> Self;
}

#[doc(hidden)]
pub trait VertexInterfaceVisitor<D: VertexDomain> {
    fn accept<V: Vertex<D>>(&mut self, path: &str, vertex: &D::Vertex<V>);
}

/// Types that are allowed to occur in a [`VertexInterface`].
#[sealed]
pub trait VertexInterfaceField<D: VertexDomain> {
    #[doc(hidden)]
    fn shader_input(path: &str) -> Self;
}

/// Types that declare the vertex input interface of a shader.
pub trait VertexInterface<D: VertexDomain> {
    type InGl: VertexInterface<Gl>;
    type InSl: VertexInterface<Sl>;

    #[doc(hidden)]
    fn visit(&self, visitor: &mut impl VertexInterfaceVisitor<D>);

    #[doc(hidden)]
    fn shader_input(path: &str) -> Self;
}

/// Provides types for declaring fields in a [`VertexInterface`].
#[sealed]
pub trait VertexDomain: Domain {
    type Vertex<V: Vertex<Self>>: VertexInterfaceField<Self>;
}

// Resource interface

#[doc(hidden)]
pub trait ResourceInterfaceVisitor<D: ResourceDomain> {
    fn accept_sampler2d<T: Numeric>(&mut self, path: &str, vertex: &D::Sampler2d<T>);

    fn accept_uniform<U: Uniform<D>>(&mut self, path: &str, vertex: &D::Uniform<U>);
}

/// Types that declare the resource input interface of a shader.
pub trait ResourceInterface<D: ResourceDomain> {
    type InGl: ResourceInterface<Gl>;
    type InSl: ResourceInterface<Sl>;

    #[doc(hidden)]
    fn visit(&self, path: &str, visitor: &mut impl ResourceInterfaceVisitor<D>);

    #[doc(hidden)]
    fn shader_input(path: &str) -> Self;
}

/// Provides types for declaring fields in a [`ResourceInterface`].
#[sealed]
pub trait ResourceDomain: Domain {
    type Sampler2d<T: Numeric>: ResourceInterface<Self>;

    type Uniform<U: Uniform<Self>>: ResourceInterface<Self>;
}

// Fragment interface

/// Types that declare the fragment output interface of a shader.
pub trait FragmentInterface<D: FragmentDomain> {
    type InGl: FragmentInterface<Gl>;
    type InSl: FragmentInterface<Sl>;
}

/// Provides types for declaring fields in a [`FragmentInterface`].
#[sealed]
pub trait FragmentDomain: Sized {
    type Attachment2d: FragmentInterface<Self>;
}
