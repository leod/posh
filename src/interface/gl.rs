use bytemuck::{Pod, Zeroable};
use sealed::sealed;

use crate::{
    dag::{BaseType, NumericType, PrimitiveType, Type},
    gl::{Sampler2dBinding, Texture2dBinding, UniformBufferBinding, VertexBufferBinding},
    sl, Gl, Numeric, Sl,
};

use super::{
    FragmentInterface, Primitive, ResourceInterface, ToPod, Uniform, Vertex, VertexAttribute,
    VertexInterface, VertexInterfaceVisitor,
};

// Uniform interface

impl Uniform<Gl> for bool {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
}

impl Uniform<Gl> for f32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
}

impl Uniform<Gl> for i32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
}

impl Uniform<Gl> for u32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
}

impl<T: Primitive> Uniform<Gl> for mint::Vector2<T> {
    type InGl = T::Vec2;
    type InSl = sl::Vec2<T>;
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

#[derive(Clone, Copy, Eq, PartialEq, Pod, Zeroable)]
#[repr(transparent)]
pub struct BoolField(u32);

impl ToPod for bool {
    type Output = BoolField;

    fn to_pod(self) -> Self::Output {
        BoolField(self as u32)
    }
}

impl ToPod for f32 {
    type Output = Self;

    fn to_pod(self) -> Self::Output {
        self
    }
}

impl ToPod for i32 {
    type Output = Self;

    fn to_pod(self) -> Self::Output {
        self
    }
}

impl ToPod for u32 {
    type Output = Self;

    fn to_pod(self) -> Self::Output {
        self
    }
}

impl<T: Primitive> ToPod for mint::Vector2<T> {
    type Output = [<T as ToPod>::Output; 2];

    fn to_pod(self) -> Self::Output {
        [self.x.to_pod(), self.y.to_pod()]
    }
}

impl Vertex<Gl> for bool {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;

    fn attributes(path: &mut Vec<&'static str>) -> Vec<VertexAttribute> {
        vec![VertexAttribute {
            name: path.join("_") + "_attr",
            ty: Type::Base(BaseType::Scalar(PrimitiveType::Numeric(
                <Self as Primitive>::NUMERIC_REPR_TYPE,
            ))),
            offset: 0,
        }]
    }
}

impl Vertex<Gl> for f32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;

    fn attributes(path: &mut Vec<&'static str>) -> Vec<VertexAttribute> {
        vec![VertexAttribute {
            name: path.join("_") + "_attr",
            ty: Type::Base(BaseType::Scalar(PrimitiveType::Numeric(
                <Self as Primitive>::NUMERIC_REPR_TYPE,
            ))),
            offset: 0,
        }]
    }
}

impl Vertex<Gl> for i32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;

    fn attributes(path: &mut Vec<&'static str>) -> Vec<VertexAttribute> {
        vec![VertexAttribute {
            name: path.join("_") + "_attr",
            ty: Type::Base(BaseType::Scalar(PrimitiveType::Numeric(
                <Self as Primitive>::NUMERIC_REPR_TYPE,
            ))),
            offset: 0,
        }]
    }
}

impl Vertex<Gl> for u32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;

    fn attributes(path: &mut Vec<&'static str>) -> Vec<VertexAttribute> {
        vec![VertexAttribute {
            name: path.join("_") + "_attr",
            ty: Type::Base(BaseType::Scalar(PrimitiveType::Numeric(
                <Self as Primitive>::NUMERIC_REPR_TYPE,
            ))),
            offset: 0,
        }]
    }
}

impl<T: Primitive> Vertex<Gl> for mint::Vector2<T> {
    type InGl = T::Vec2;
    type InSl = sl::Vec2<T>;

    fn attributes(path: &mut Vec<&'static str>) -> Vec<VertexAttribute> {
        vec![VertexAttribute {
            name: path.join("_") + "_attr",
            ty: Type::Base(BaseType::Vec2(PrimitiveType::Numeric(T::NUMERIC_REPR_TYPE))),
            offset: 0,
        }]
    }
}

impl<V: Vertex<Gl>> VertexInterface<Gl> for VertexBufferBinding<V> {
    type InGl = Self;
    type InSl = V::InSl;

    fn visit(&self, path: &mut Vec<&'static str>, visitor: &mut impl VertexInterfaceVisitor<Gl>) {
        visitor.accept(path, self)
    }
}

#[sealed]
impl super::VertexDomain for Gl {
    type Vertex<V: Vertex<Gl>> = VertexBufferBinding<V>;
}

// Resource interface

impl<T: Numeric> ResourceInterface<Gl> for Sampler2dBinding<T> {
    type InGl = Self;
    type InSl = sl::Sampler2d<T>;
}

impl<U: Uniform<Sl>> ResourceInterface<Gl> for UniformBufferBinding<U> {
    type InGl = Self;
    type InSl = U::InSl;
}

#[sealed]
impl super::ResourceDomain for Gl {
    type Sampler2d<T: Numeric> = Sampler2dBinding<T>;
    type Uniform<U: Uniform<Gl>> = UniformBufferBinding<U::InSl>;
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
