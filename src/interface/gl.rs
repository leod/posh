use std::process::Output;

use bytemuck::{Pod, Zeroable};
use sealed::sealed;

use crate::{
    gl::{Sampler2dBinding, Texture2dBinding, UniformBufferBinding, VertexBufferBinding},
    sl::{self, Object},
    Gl, Numeric, Sl, VertexInputRate,
};

use super::{
    FragmentInterface, Primitive, ResourceInterface, ToPod, Uniform, Vertex, VertexAttribute,
    VertexInterface, VertexInterfaceVisitor,
};

// Uniform interface

impl Uniform<Gl> for bool {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl Uniform<Gl> for f32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl Uniform<Gl> for i32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl Uniform<Gl> for u32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl<T: Primitive> Uniform<Gl> for mint::Vector2<T> {
    type InGl = T::Vec2;
    type InSl = sl::Vec2<T>;

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

#[sealed]
impl super::Domain for Gl {
    type Scalar<T: Primitive> = T;
    type Vec2<T: Primitive> = mint::Vector2<T>;

    type Bool = bool;
    type F32 = f32;
    type I32 = i32;
    type U32 = u32;
}

// Vertex interface

impl Vertex<Gl> for bool {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
    type Pod = <Self::InGl as ToPod>::Output;

    fn attributes(path: &str) -> Vec<VertexAttribute> {
        <Self::InSl as Vertex<Sl>>::attributes(path)
    }

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl Vertex<Gl> for f32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
    type Pod = <Self::InGl as ToPod>::Output;

    fn attributes(path: &str) -> Vec<VertexAttribute> {
        <Self::InSl as Vertex<Sl>>::attributes(path)
    }

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl Vertex<Gl> for i32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
    type Pod = <Self::InGl as ToPod>::Output;

    fn attributes(path: &str) -> Vec<VertexAttribute> {
        <Self::InSl as Vertex<Sl>>::attributes(path)
    }

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl Vertex<Gl> for u32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
    type Pod = <Self::InGl as ToPod>::Output;

    fn attributes(path: &str) -> Vec<VertexAttribute> {
        <Self::InSl as Vertex<Sl>>::attributes(path)
    }

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl<T: Primitive> Vertex<Gl> for mint::Vector2<T> {
    type InGl = T::Vec2;
    type InSl = sl::Vec2<T>;
    type Pod = <Self::InGl as ToPod>::Output;

    fn attributes(path: &str) -> Vec<VertexAttribute> {
        <Self::InSl as Vertex<Sl>>::attributes(path)
    }

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl<V: Vertex<Sl>> VertexInterface<Gl> for VertexBufferBinding<V> {
    type InGl = Self;
    type InSl = V::InSl;

    fn visit(&self, visitor: &mut impl VertexInterfaceVisitor<Gl>) {
        visitor.accept("vertex", VertexInputRate::Vertex, self)
    }

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

#[sealed]
impl<V: Vertex<Sl>> super::VertexInterfaceField<Gl> for VertexBufferBinding<V> {
    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

#[sealed]
impl super::VertexDomain for Gl {
    type Vertex<V: Vertex<Sl>> = VertexBufferBinding<V>;
}

// Resource interface

impl<T: Numeric> ResourceInterface<Gl> for Sampler2dBinding<T> {
    type InGl = Self;
    type InSl = sl::Sampler2d<T>;

    fn visit(&self, path: &str, visitor: &mut impl super::ResourceInterfaceVisitor<Gl>) {
        visitor.accept_sampler2d(path, self);
    }

    fn shader_input(_: &str) -> Self {
        unimplemented!()
    }
}

impl<U: Uniform<Sl, InSl = U>> ResourceInterface<Gl> for UniformBufferBinding<U> {
    type InGl = Self;
    type InSl = U;

    fn visit(&self, path: &str, visitor: &mut impl super::ResourceInterfaceVisitor<Gl>) {
        visitor.accept_uniform::<U::InSl>(path, self);
    }

    fn shader_input(_: &str) -> Self {
        todo!()
    }
}

#[sealed]
impl super::ResourceDomain for Gl {
    type Sampler2d<T: Numeric> = Sampler2dBinding<T>;
    type Uniform<U: Uniform<Sl, InSl = U>> = UniformBufferBinding<U>;
    type Compose<R: ResourceInterface<Sl>> = R::InGl;
}

// Fragment interface

impl FragmentInterface<Gl> for Texture2dBinding {
    type InGl = Self;
    type InSl = sl::Vec4<f32>;
}

#[sealed]
impl super::FragmentDomain for Gl {
    type Attachment2d = Texture2dBinding;
}
