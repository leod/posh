use sealed::sealed;

use crate::{
    sl::{Sampler2d, Scalar, Vec2},
    Numeric, Sl, Uniform,
};

use super::{Attributes, Primitive, Resource, ResourceDomain, Vertex};

#[sealed]
impl<T: Primitive> super::UniformField<Sl> for Scalar<T> {}

#[sealed]
impl<T: Primitive> super::UniformField<Sl> for Vec2<T> {}

#[sealed]
impl super::UniformDomain for Sl {
    type F32 = Scalar<f32>;
    type I32 = Scalar<i32>;
    type U32 = Scalar<u32>;
    type Vec2<T: Primitive> = Vec2<T>;
}

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

impl<V: Vertex<Sl>> Attributes<Sl> for V {
    type InSl = V::InSl;
}

#[sealed]
impl super::AttributesDomain for Sl {
    type Vertex<V: Vertex<Sl>> = V;
}

impl<T: Numeric> Resource<Sl> for Sampler2d<T> {
    type InSl = Self;
}

impl<U: Uniform<Sl>> Resource<Sl> for U {
    type InSl = Self;
}

impl ResourceDomain for Sl {
    type Sampler2d<T: Numeric> = Sampler2d<T>;
    type Uniform<U: Uniform<Sl>> = U;
}
