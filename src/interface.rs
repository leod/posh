use bytemuck::Pod;
use crevice::std140::AsStd140;
use sealed::sealed;

mod gl;
mod sl;

use crate::{
    sl::{Bool, Scalar, ToValue, Value, Vec2, F32, I32, U32},
    Gl, Numeric, Primitive, Sl,
};

/// Provides types for defining fields in custom [`Uniform`] or [`Vertex`]
/// structs.
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

/// Vertex types.
pub trait Vertex<D: Domain>: ToValue {
    type InGl: Vertex<Gl> + ToPod + ToValue<Output = Self::InSl>;
    type InSl: Vertex<Sl> + Value + ToValue<Output = Self::InSl>;
}

/// Provides types for defining fields in custom [`VertexInterface`] structs.
#[sealed]
pub trait VertexDomain: Domain {
    type Vertex<V: Vertex<Self>>;
}

/// Types that define a shader vertex input interface.
pub trait VertexInterface<D: VertexDomain> {
    type InGl: VertexInterface<Gl>;
    type InSl: VertexInterface<Sl>;
}

// Resource interface

/// Resource types.
pub trait Resource<D: ResourceDomain> {
    type InGl: Resource<Gl>;
    type InSl: Resource<Sl>;
}

/// Provides types for defining fields in custom [`ResourceInterface`] structs.
pub trait ResourceDomain: Domain {
    type Sampler2d<T: Numeric>: Resource<Self>;

    type Uniform<U: Uniform<Self>>: Resource<Self>;
}

impl<D, V, W> Resource<D> for (V, W)
where
    D: ResourceDomain,
    V: Resource<D>,
    W: Resource<D>,
{
    type InGl = (V::InGl, W::InGl);
    type InSl = (V::InSl, W::InSl);
}

/// Types that define a shader resource input interface.
pub trait ResourceInterface<D: ResourceDomain> {
    type InGl: ResourceInterface<Gl>;
    type InSl: ResourceInterface<Sl>;
}

// Fragment interface

/// Provides types for defining fields in custom [`FragmentInterface`] structs.
pub trait FragmentDomain: Sized {
    type Attachment2d: Fragment<Self>;
}

/// Fragment types.
pub trait Fragment<D: FragmentDomain> {
    type InGl: Fragment<Gl>;
    type InSl: Fragment<Sl>;
}

/// Types that define a shader fragment output interface.
pub trait FragmentInterface<D: FragmentDomain> {
    type InGl: FragmentInterface<Gl>;
    type InSl: FragmentInterface<Sl>;
}
