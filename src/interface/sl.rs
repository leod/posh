use sealed::sealed;

use crate::{
    gl::{self, Texture2dBinding},
    sl::{Sampler2d, Scalar, Vec2, Vec4},
    Gl, Numeric, Sl,
};

use super::{
    FragmentInterface, Primitive, ResourceInterface, Uniform, Vertex, VertexAttribute,
    VertexInterface, VertexInterfaceVisitor,
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

    fn attributes(path: &mut Vec<&'static str>) -> Vec<VertexAttribute> {
        Self::InGl::attributes(path)
    }
}

impl<T: Primitive> Vertex<Sl> for Vec2<T> {
    type InGl = T::Vec2;
    type InSl = Self;

    fn attributes(path: &mut Vec<&'static str>) -> Vec<VertexAttribute> {
        Self::InGl::attributes(path)
    }
}

// Vertex interface

impl<V: Vertex<Sl>> VertexInterface<Sl> for V {
    type InGl = gl::VertexBufferBinding<V::InGl>;
    type InSl = V::InSl;

    fn visit(&self, path: &mut Vec<&'static str>, visitor: &mut impl VertexInterfaceVisitor<Sl>) {
        visitor.accept(path, self)
    }
}

#[sealed]
impl super::VertexDomain for Sl {
    type Vertex<V: Vertex<Sl>> = V;
}

// Resource interface

impl<T: Numeric> ResourceInterface<Sl> for Sampler2d<T> {
    type InGl = gl::Sampler2dBinding<T>;
    type InSl = Self;
}

impl<U: Uniform<Sl>> ResourceInterface<Sl> for U {
    type InGl = gl::UniformBufferBinding<U>;
    type InSl = Self;
}

#[sealed]
impl super::ResourceDomain for Sl {
    type Sampler2d<T: Numeric> = Sampler2d<T>;
    type Uniform<U: Uniform<Sl>> = U;
}

// Fragment interface

impl FragmentInterface<Sl> for Vec4<f32> {
    type InGl = Texture2dBinding;
    type InSl = Self;
}

#[sealed]
impl super::FragmentDomain for Sl {
    type Attachment2d = Vec4<f32>;
}
