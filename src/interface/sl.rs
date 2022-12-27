use sealed::sealed;

use crate::{
    gl,
    sl::{Sampler2d, Scalar, Vec2, Vec4},
    Numeric, Sl,
};

use super::{
    Fragment, FragmentDomain, Primitive, Resource, ResourceDomain, Uniform, Vertex, VertexInterface,
};

// Uniform interface

impl<T: Primitive> Uniform<Sl> for Scalar<T> {
    type InGl = T;
    type InSl = Self;
}

impl<T: Primitive> Uniform<Sl> for Vec2<T> {
    type InGl = T::Vec2;
    type InSl = Self;
}

#[sealed]
impl super::Domain for Sl {
    type Scalar<T: Primitive> = Scalar<T>;
    type Vec2<T: Primitive> = Vec2<T>;

    type Bool = Scalar<bool>;
    type F32 = Scalar<f32>;
    type I32 = Scalar<i32>;
    type U32 = Scalar<u32>;
}

// Vertex interface

impl<T: Primitive> Vertex<Sl> for Scalar<T> {
    type InGl = T;
    type InSl = Self;
}

impl<T: Primitive> Vertex<Sl> for Vec2<T> {
    type InGl = T::Vec2;
    type InSl = Self;
}

// Attributes interface

impl<V: Vertex<Sl>> VertexInterface<Sl> for V {
    type InGl = gl::VertexBufferBinding<V::InGl>;
    type InSl = V::InSl;
}

#[sealed]
impl super::VertexDomain for Sl {
    type Vertex<V: Vertex<Sl>> = V;
}

// Resource interface

impl<T: Numeric> Resource<Sl> for Sampler2d<T> {
    type InGl = gl::Sampler2dBinding<T>;
    type InSl = Self;
}

impl<U: Uniform<Sl>> Resource<Sl> for U {
    type InGl = gl::UniformBufferBinding<U>;
    type InSl = Self;
}

impl ResourceDomain for Sl {
    type Sampler2d<T: Numeric> = Sampler2d<T>;
    type Uniform<U: Uniform<Sl>> = U;
}

// Fragment interface

impl Fragment<Sl> for Vec4<f32> {
    type InGl = gl::Texture2dBinding;
    type InSl = Self;
}

impl FragmentDomain for Sl {
    type Attachment2d = Vec4<f32>;
}
