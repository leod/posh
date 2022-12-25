use bytemuck::Pod;
use crevice::std140::AsStd140;
use sealed::sealed;

mod gl;
mod sl;

use crate::{
    sl::{Object, Value},
    Gl, Numeric, Primitive, Sl,
};

// Uniform interface

#[sealed]
pub trait UniformDomain: Copy {
    /// A scalar value.
    type Scalar<T: Primitive>: Uniform<Self>;

    /// A two-dimensional vector.
    type Vec2<T: Primitive>: Uniform<Self>;

    /// A boolean value.
    ///
    /// Same as [`Self::Scalar<bool>`].
    type Bool: Uniform<Self>;

    /// A floating-point value.
    ///
    /// Same as [`Self::Scalar<f32>`].
    type F32: Uniform<Self>;

    /// A signed integer value.
    ///
    /// Same as [`Self::Scalar<i32>`].
    type I32: Uniform<Self>;

    /// An unsigned integer value.
    ///
    /// Same as [`Self::Scalar<u32>`].
    type U32: Uniform<Self>;
}

/// A type that can be used as uniform input for shaders.
pub trait Uniform<D: UniformDomain>: Copy {
    type InGl: Uniform<Gl> + AsStd140;
    type InSl: Uniform<Sl> + Value;
}

// Vertex interface

pub trait AsPod {
    type Pod: Pod;
}

/// Allowed types for fields in a [`Vertex`].
///
/// According to the specification **GLSL 3.30, 4.3.4**:
/// > Vertex shader inputs can only be `float`, floating-point vectors,
/// > matrices, signed and unsigned integers and integer vectors. Vertex shader
/// > inputs can also form arrays of these types, but not structures.
///
/// The interface of this trait is a private implementation detail.
#[sealed]
pub trait VertexField<D: VertexDomain> {}

#[sealed]
pub trait VertexDomain: Sized {
    /// A floating-point value.
    type F32: VertexField<Self>;

    /// A signed integer value.
    type I32: VertexField<Self>;

    /// An unsigned integer value.
    type U32: VertexField<Self>;

    /// A two-dimensional vector.
    type Vec2<T: Numeric>: VertexField<Self>;
}

/// A type that can be used as vertex input for shaders.
pub trait Vertex<D: VertexDomain> {
    type InGl: Vertex<Gl> + AsPod;
    type InSl: Vertex<Sl> + Value;
}

// Attributes interface

/// A type that can be used as attributes input for shaders.
pub trait Attributes<D: AttributesDomain> {
    type InGl: Attributes<Gl>;
    type InSl: Attributes<Sl> + Value;
}

#[sealed]
pub trait AttributesDomain: VertexDomain {
    type Vertex<V: Vertex<Self>>: Attributes<Self>;
}

impl<D, V, W> Attributes<D> for (V, W)
where
    D: AttributesDomain,
    V: Attributes<D>,
    W: Attributes<D>,
{
    type InGl = (V::InGl, W::InGl);
    type InSl = (V::InSl, W::InSl);
}

// Resource interface

/// A type that can be used as resource input for shaders.
pub trait Resource<D: ResourceDomain> {
    type InGl: Resource<Gl>;
    type InSl: Resource<Sl>;
}

pub trait ResourceDomain: UniformDomain {
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

// Fragment interface

pub trait Attachment<D: FragmentDomain> {}

pub trait FragmentDomain: Sized {
    type Attachment2d: Attachment<Self>;
}

/// A type that can be used as fragment output for shaders.
pub trait Fragment<D: FragmentDomain> {
    type InGl: Fragment<Gl>;
    type InSl: Fragment<Sl> + Value;
}
