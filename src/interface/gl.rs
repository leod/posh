use sealed::sealed;

use crate::{
    gl::{Sampler2d, Texture2dBinding, UniformBufferBinding, VertexBufferBinding},
    sl, Gl, Numeric, Sl, Uniform,
};

use super::{Attachment, Attributes, FragmentDomain, Primitive, Resource, ResourceDomain, Vertex};

// Uniform interface

#[sealed]
impl super::UniformField<Gl> for f32 {}

#[sealed]
impl super::UniformField<Gl> for i32 {}

#[sealed]
impl super::UniformField<Gl> for u32 {}

#[sealed]
impl<T: Primitive> super::UniformField<Gl> for mint::Vector2<T> {}

#[sealed]
impl super::UniformDomain for Gl {
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

impl<V: Vertex<Sl>> Attributes<Gl> for VertexBufferBinding<V> {
    type InSl = V::InSl;
}

#[sealed]
impl super::AttributesDomain for Gl {
    type Vertex<V: Vertex<Gl>> = VertexBufferBinding<V::InSl>;
}

// Resource interface

impl<T: Numeric> Resource<Gl> for Sampler2d<T> {
    type InSl = sl::Sampler2d<T>;
}

impl<U: Uniform<Sl>> Resource<Gl> for UniformBufferBinding<U> {
    type InSl = U::InSl;
}

impl ResourceDomain for Gl {
    type Sampler2d<T: Numeric> = Sampler2d<T>;
    type Uniform<U: Uniform<Gl>> = UniformBufferBinding<U::InSl>;
}

// Fragment interface

impl Attachment<Gl> for Texture2dBinding {}

impl FragmentDomain for Gl {
    type Attachment2d = Texture2dBinding;
}
