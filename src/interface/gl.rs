use sealed::sealed;

use crate::{
    gl::{Sampler2dBinding, Texture2dBinding, UniformBufferBinding, VertexBufferBinding},
    sl::{self, Scalar, Vec2},
    Gl, Numeric, Sl, Uniform,
};

use super::{Attachment, Attributes, FragmentDomain, Primitive, Resource, ResourceDomain, Vertex};

// Uniform interface

impl<T: Primitive> Uniform<Gl> for T {
    type InGl = Self::InGl;
    type InSl = Scalar<Self>;
}

impl<T: Primitive> Uniform<Gl> for mint::Vector2<T> {
    type InGl = T::Vec2;
    type InSl = Vec2<T>;
}

#[sealed]
impl super::UniformDomain for Gl {
    type Bool = bool;
    type F32 = f32;
    type I32 = i32;
    type U32 = u32;
    type Vec2<T: Primitive> = mint::Vector2<T>;
}

// Vertex interface

#[sealed]
impl<T: Numeric> super::VertexField<Gl> for T {}

#[sealed]
impl<T: Numeric> super::VertexField<Gl> for mint::Vector2<T> {}

#[sealed]
impl super::VertexDomain for Gl {
    type F32 = f32;
    type I32 = i32;
    type U32 = u32;
    type Vec2<T: Numeric> = T;
}

// Attributes interface

impl<V: Vertex<Gl>> Attributes<Gl> for VertexBufferBinding<V> {
    type InGl = Self;
    type InSl = V::InSl;
}

#[sealed]
impl super::AttributesDomain for Gl {
    type Vertex<V: Vertex<Gl>> = VertexBufferBinding<V>;
}

// Resource interface

impl<T: Numeric> Resource<Gl> for Sampler2dBinding<T> {
    type InGl = Self;
    type InSl = sl::Sampler2d<T>;
}

impl<U: Uniform<Sl>> Resource<Gl> for UniformBufferBinding<U> {
    type InGl = Self;
    type InSl = U::InSl;
}

impl ResourceDomain for Gl {
    type Sampler2d<T: Numeric> = Sampler2dBinding<T>;
    type Uniform<U: Uniform<Gl>> = UniformBufferBinding<U::InSl>;
}

// Fragment interface

impl Attachment<Gl> for Texture2dBinding {}

impl FragmentDomain for Gl {
    type Attachment2d = Texture2dBinding;
}
