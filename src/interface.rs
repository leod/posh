use bytemuck::Pod;
use sealed::sealed;

mod gl;
mod sl;

use crate::{sl::Value, Gl, Numeric, Primitive, Sl};

// Uniform interface

/// Allowed types for fields in a [`Uniform`].
///
/// The interface of this trait is a private implementation detail.
#[sealed]
pub trait UniformField<D: UniformDomain> {}

#[sealed]
impl<D: UniformDomain, U: Uniform<D>> UniformField<D> for U {}

#[sealed]
pub trait UniformDomain: Sized {
    /// A floating-point value.
    type F32: UniformField<Self>;

    /// A signed integer value.
    type I32: UniformField<Self>;

    /// An unsigned integer value.
    type U32: UniformField<Self>;

    /// A two-dimensional vector.
    type Vec2<T: Primitive>: UniformField<Self>;
}

/// A type that can be used as uniform input for shaders.
pub trait Uniform<D: UniformDomain> {
    type InGl: Uniform<Gl>; //+ AsStd140;
    type InSl: Uniform<Sl> + Value;
}

// Vertex interface

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
    type InGl: Vertex<Gl> + Pod;
    type InSl: Vertex<Sl> + Value;
}

// Attributes interface

/// A type that can be used as attributes input for shaders.
pub trait Attributes<D: AttributesDomain> {
    type InSl: Attributes<Sl>;
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
    type InSl = (V::InSl, W::InSl);
}

// Resource interface

/// A type that can be used as resource input for shaders.
pub trait Resource<I: ResourceDomain> {
    type InSl: Resource<Sl>;
}

pub trait ResourceDomain: UniformDomain {
    type Sampler2d<T: Numeric>: Resource<Self>;

    type Uniform<U: Uniform<Self>>: Resource<Self>;
}

impl<I, V, W> Resource<I> for (V, W)
where
    I: ResourceDomain,
    V: Resource<I>,
    W: Resource<I>,
{
    type InSl = (V::InSl, W::InSl);
}

// Fragment interface

pub trait Attachment<D: FragmentDomain> {}

pub trait FragmentDomain: Sized {
    type Attachment2d: Attachment<Self>;
}
