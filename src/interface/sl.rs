use sealed::sealed;

use crate::{
    dag::{BaseType, NumericType, PrimitiveType, Type},
    gl::{self, Texture2dBinding},
    program_def::{VertexAttributeDef, VertexInputRate},
    sl::{Mat2, Mat3, Mat4, Object, Sampler2d, Scalar, Vec2, Vec3, Vec4},
    Numeric, Sl,
};

use super::{
    Block, FragmentInterface, FragmentInterfaceVisitor, Primitive, ResourceInterface,
    VertexInterface, VertexInterfaceVisitor,
};

// Block

#[sealed]
impl super::BlockDomain for Sl {
    type Scalar<T: Primitive> = Scalar<T>;
    type Vec2<T: Primitive> = Vec2<T>;
    type Vec3<T: Primitive> = Vec3<T>;
    type Vec4<T: Primitive> = Vec4<T>;
    type Mat2 = Mat2;
    type Mat3 = Mat3;
    type Mat4 = Mat4;

    type Bool = Scalar<bool>;
    type F32 = Scalar<f32>;
    type I32 = Scalar<i32>;
    type U32 = Scalar<u32>;
}

unsafe impl<T: Primitive> Block<Sl> for Scalar<T> {
    type InGl = T;
    type InSl = Self;

    fn uniform_input(path: &str) -> Self {
        <Self as Object>::from_arg(path)
    }

    fn vertex_input(path: &str) -> Self {
        if T::PRIMITIVE_TYPE != PrimitiveType::Bool {
            <Self as Object>::from_arg(path)
        } else {
            Scalar::<u32>::from_arg(path).cast()
        }
    }

    fn vertex_attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
        vec![VertexAttributeDef {
            name: path.to_string(),
            ty: Type::Base(BaseType::Scalar(vertex_attribute_numeric_repr(
                T::PRIMITIVE_TYPE,
            ))),
            offset: 0,
        }]
    }
}

macro_rules! impl_block_for_vec {
    ($ty:ident) => {
        unsafe impl<T: Primitive> Block<Sl> for $ty<T> {
            type InGl = T::$ty;
            type InSl = Self;

            fn uniform_input(path: &str) -> Self {
                <Self as Object>::from_arg(path)
            }

            fn vertex_input(path: &str) -> Self {
                if T::PRIMITIVE_TYPE != PrimitiveType::Bool {
                    <Self as Object>::from_arg(path)
                } else {
                    $ty::<u32>::from_arg(path).cast()
                }
            }

            fn vertex_attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
                vec![VertexAttributeDef {
                    name: path.to_string(),
                    ty: Type::Base(BaseType::$ty(vertex_attribute_numeric_repr(
                        T::PRIMITIVE_TYPE,
                    ))),
                    offset: 0,
                }]
            }
        }
    };
}

impl_block_for_vec!(Vec2);
impl_block_for_vec!(Vec3);
impl_block_for_vec!(Vec4);

macro_rules! impl_block_for_mat {
    ($ty:ident, $mint_ty:ident) => {
        unsafe impl Block<Sl> for $ty {
            type InGl = mint::$mint_ty<f32>;
            type InSl = Self;

            fn uniform_input(path: &str) -> Self {
                <Self as Object>::from_arg(path)
            }

            fn vertex_input(path: &str) -> Self {
                <Self as Object>::from_arg(path)
            }

            fn vertex_attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
                vec![VertexAttributeDef {
                    name: path.to_string(),
                    ty: Type::Base(BaseType::$ty),
                    offset: 0,
                }]
            }
        }
    };
}

impl_block_for_mat!(Mat2, ColumnMatrix2);
impl_block_for_mat!(Mat3, ColumnMatrix3);
impl_block_for_mat!(Mat4, ColumnMatrix4);

/// Returns the type that is used to specify a value of a given primitive type
/// in vertex arrays.
///
/// This exists because OpenGL does not like `bool`s in attributes, but, for
/// completeness, we want to be able to provide the same basic types in uniforms
/// as in vertices. Among other things, this allows deriving both `Vertex` and
/// `Uniform` for the same `struct`, which might be useful for supporting
/// different instancing methods for the same data.
fn vertex_attribute_numeric_repr(ty: PrimitiveType) -> PrimitiveType {
    use PrimitiveType::*;

    PrimitiveType::Numeric(match ty {
        Numeric(ty) => ty,
        Bool => NumericType::U32,
    })
}

// VertexInterface

#[sealed]
impl super::VertexDomain for Sl {
    type Vertex<V: Block<Sl>> = V;
}

unsafe impl<V: Block<Sl>> VertexInterface<Sl> for V {
    type InGl = gl::VertexBuffer<V>;
    type InSl = V::InSl;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexInterfaceVisitor<'a, Sl>) {
        visitor.accept(path, VertexInputRate::Vertex, self);
    }

    fn shader_input(path: &str) -> Self {
        V::vertex_input(path)
    }
}

#[sealed]
impl<V: Block<Sl>> super::VertexInterfaceField<Sl> for V {
    fn shader_input(path: &str) -> Self {
        V::vertex_input(path)
    }
}

// ResourceInterface

#[sealed]
impl super::ResourceDomain for Sl {
    type Sampler2d<T: Numeric> = Sampler2d<T>;
    type Uniform<U: Block<Sl, InSl = U>> = U;
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

unsafe impl<U: Block<Sl, InSl = U>> ResourceInterface<Sl> for U {
    type InGl = gl::UniformBufferBinding<U>;
    type InSl = Self;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::ResourceInterfaceVisitor<'a, Sl>) {
        visitor.accept_uniform(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <U as Block<Sl>>::uniform_input(path)
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
