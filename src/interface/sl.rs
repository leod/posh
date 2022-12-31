use sealed::sealed;

use crate::{
    dag::{BaseType, PrimitiveType, Type},
    gl::{self, Texture2dBinding},
    sl::{primitives::input, Sampler2d, Scalar, Vec2, Vec4},
    Numeric, Sl,
};

use super::{
    join_ident_path, FragmentInterface, Primitive, ResourceInterface, Uniform, Vertex,
    VertexAttribute, VertexInterface, VertexInterfaceVisitor,
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

fn vertex_attribute(path: &str, base_type: BaseType) -> Vec<VertexAttribute> {
    vec![VertexAttribute {
        name: super::join_ident_path(path, "attr"),
        ty: Type::Base(base_type),
        offset: 0,
    }]
}

impl<T: Primitive> Vertex<Sl> for Scalar<T> {
    type InGl = T;
    type InSl = Self;

    fn attributes(path: &str) -> Vec<VertexAttribute> {
        vertex_attribute(
            path,
            BaseType::Scalar(PrimitiveType::Numeric(T::NUMERIC_REPR_TYPE)),
        )
    }

    fn shader_input(path: &str) -> Self {
        input(&join_ident_path(path, "attr"))
    }
}

impl<T: Primitive> Vertex<Sl> for Vec2<T> {
    type InGl = T::Vec2;
    type InSl = Self;

    fn attributes(path: &str) -> Vec<VertexAttribute> {
        vertex_attribute(
            path,
            BaseType::Vec2(PrimitiveType::Numeric(T::NUMERIC_REPR_TYPE)),
        )
    }

    fn shader_input(path: &str) -> Self {
        input(&join_ident_path(path, "attr"))
    }
}

// Vertex interface

impl<V: Vertex<Sl>> VertexInterface<Sl> for V {
    type InGl = gl::VertexBufferBinding<V::InGl>;
    type InSl = V::InSl;

    fn visit(&self, visitor: &mut impl VertexInterfaceVisitor<Sl>) {
        visitor.accept("vertex", self);
    }

    fn shader_input(path: &str) -> Self {
        V::shader_input(path)
    }
}

#[sealed]
impl<V: Vertex<Sl>> super::VertexInterfaceField<Sl> for V {
    fn shader_input(path: &str) -> Self {
        V::shader_input(path)
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
