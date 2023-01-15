use sealed::sealed;

use crate::{
    dag::{BaseType, NumericType, PrimitiveType, Type},
    gl::{self, Texture2dBinding},
    program_def::{VertexAttributeDef, VertexInputRate},
    sl::{Object, Sampler2d, Scalar, Vec2, Vec3, Vec4},
    Numeric, Sl,
};

use super::{
    FragmentInterface, FragmentInterfaceVisitor, Primitive, ResourceInterface, Uniform, Vertex,
    VertexInterface, VertexInterfaceVisitor,
};

#[sealed]
impl super::Domain for Sl {
    type Scalar<T: Primitive> = Scalar<T>;
    type Vec2<T: Primitive> = Vec2<T>;
    type Vec3<T: Primitive> = Vec3<T>;
    type Vec4<T: Primitive> = Vec4<T>;

    type Bool = Scalar<bool>;
    type F32 = Scalar<f32>;
    type I32 = Scalar<i32>;
    type U32 = Scalar<u32>;
}

// Uniform

unsafe impl<T: Primitive> Uniform<Sl> for Scalar<T> {
    type InGl = T;
    type InSl = Self;

    fn shader_input(path: &str) -> Self {
        <Self as Object>::from_arg(path)
    }
}

macro_rules! impl_uniform {
    ($ty:ident) => {
        unsafe impl<T: Primitive> Uniform<Sl> for $ty<T> {
            type InGl = T::$ty;
            type InSl = Self;

            fn shader_input(path: &str) -> Self {
                <Self as Object>::from_arg(path)
            }
        }
    };
}

impl_uniform!(Vec2);
impl_uniform!(Vec3);
impl_uniform!(Vec4);

// Vertex

/// Returns the type that is used to specify a value of a given primitive type
/// in vertex arrays.
///
/// This exists because OpenGL does not like `bool`s in attributes, but, for
/// completeness, we want to be able to provide the same basic types in uniforms
/// as in vertices. Among other things, this allows deriving both `Vertex` and
/// `Uniform` for the same `struct`, which might be useful for supporting
/// different instancing methods for the same data.
fn numeric_repr(ty: PrimitiveType) -> PrimitiveType {
    use PrimitiveType::*;

    PrimitiveType::Numeric(match ty {
        Numeric(ty) => ty,
        Bool => NumericType::U32,
    })
}

macro_rules! impl_vertex {
    ($ty:ident) => {
        unsafe impl<T: Primitive> Vertex<Sl> for $ty<T> {
            type InGl = <Self as Uniform<Sl>>::InGl;
            type InSl = Self;

            fn attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
                vec![VertexAttributeDef {
                    name: path.to_string(),
                    ty: Type::Base(BaseType::$ty(numeric_repr(T::PRIMITIVE_TYPE))),
                    offset: 0,
                }]
            }

            fn shader_input(path: &str) -> Self {
                if T::PRIMITIVE_TYPE != PrimitiveType::Bool {
                    <Self as Object>::from_arg(path)
                } else {
                    $ty::<u32>::from_arg(path).cast()
                }
            }
        }
    };
}

impl_vertex!(Scalar);
impl_vertex!(Vec2);
impl_vertex!(Vec3);
impl_vertex!(Vec4);

// VertexInterface

#[sealed]
impl super::VertexDomain for Sl {
    type Vertex<V: Vertex<Sl>> = V;
}

unsafe impl<V: Vertex<Sl>> VertexInterface<Sl> for V {
    type InGl = gl::VertexBuffer<V>;
    type InSl = V::InSl;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexInterfaceVisitor<'a, Sl>) {
        visitor.accept(path, VertexInputRate::Vertex, self);
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

// ResourceInterface

#[sealed]
impl super::ResourceDomain for Sl {
    type Sampler2d<T: Numeric> = Sampler2d<T>;
    type Uniform<U: Uniform<Sl, InSl = U>> = U;
    type Compose<R: ResourceInterface<Sl>> = R;
}

unsafe impl<T: Numeric> ResourceInterface<Sl> for Sampler2d<T> {
    type InGl = gl::Sampler2dBinding<T>;
    type InSl = Self;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::ResourceInterfaceVisitor<'a, Sl>) {
        visitor.accept_sampler2d(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <Self as Object>::from_arg(path)
    }
}

unsafe impl<U: Uniform<Sl, InSl = U>> ResourceInterface<Sl> for U {
    type InGl = gl::UniformBufferBinding<U>;
    type InSl = Self;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::ResourceInterfaceVisitor<'a, Sl>) {
        visitor.accept_uniform(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <U as Uniform<Sl>>::shader_input(path)
    }
}

// FragmentInterface

#[sealed]
impl super::FragmentDomain for Sl {
    type Attachment = Vec4<f32>;
}

unsafe impl FragmentInterface<Sl> for Vec4<f32> {
    type InGl = Texture2dBinding;
    type InSl = Self;

    fn visit(&self, path: &str, visitor: &mut impl FragmentInterfaceVisitor<Sl>) {
        visitor.accept(path, self);
    }
}
