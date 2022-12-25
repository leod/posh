use sealed::sealed;

use crate::{
    gl::{Sampler2dBinding, UniformBufferBinding, VertexBufferBinding},
    sl::{Sampler2d, Scalar, Vec2, Vec4},
    Numeric, Sl, Uniform,
};

use super::{Attachment, Attributes, FragmentDomain, Primitive, Resource, ResourceDomain, Vertex};

// Uniform interface

impl<T: Primitive> Uniform<Sl> for Scalar<T> {
    type InGl = T::InGl;
    type InSl = Self;
}

impl<T: Primitive> Uniform<Sl> for Vec2<T> {
    type InGl = T::Vec2;
    type InSl = Self;
}

#[sealed]
impl super::UniformDomain for Sl {
    type Scalar<T: Primitive> = Scalar<T>;
    type Vec2<T: Primitive> = Vec2<T>;

    type Bool = Scalar<bool>;
    type F32 = Scalar<f32>;
    type I32 = Scalar<i32>;
    type U32 = Scalar<u32>;
}

#[sealed]
impl super::UniformDomainHelper for Sl {
    type F32 = Scalar<f32>;
    type I32 = Scalar<i32>;
    type U32 = Scalar<u32>;
    type Vec2<T: Primitive> = Vec2<T>;
}

// Vertex interface

#[sealed]
impl<T: Numeric> super::VertexField<Sl> for Scalar<T> {}

#[sealed]
impl<T: Numeric> super::VertexField<Sl> for Vec2<T> {}

#[sealed]
impl super::VertexDomain for Sl {
    type F32 = Scalar<f32>;
    type I32 = Scalar<i32>;
    type U32 = Scalar<u32>;
    type Vec2<T: Numeric> = Vec2<T>;
}

// Attributes interface

impl<V: Vertex<Sl>> Attributes<Sl> for V {
    type InGl = VertexBufferBinding<V::InGl>;
    type InSl = V::InSl;
}

#[sealed]
impl super::AttributesDomain for Sl {
    type Vertex<V: Vertex<Sl>> = V;
}

// Resource interface

impl<T: Numeric> Resource<Sl> for Sampler2d<T> {
    type InGl = Sampler2dBinding<T>;
    type InSl = Self;
}

impl<U: Uniform<Sl>> Resource<Sl> for U {
    type InGl = UniformBufferBinding<U>;
    type InSl = Self;
}

impl ResourceDomain for Sl {
    type Sampler2d<T: Numeric> = Sampler2d<T>;
    type Uniform<U: Uniform<Sl>> = U;
}

// Fragment interface

impl Attachment<Sl> for Vec4<f32> {}

impl FragmentDomain for Sl {
    type Attachment2d = Vec4<f32>;
}
