use sealed::sealed;

use crate::{
    dag::{BaseType, NumericType, PrimitiveType, Type},
    gl,
    program_def::{VertexAttributeDef, VertexInputRate},
    sl::{Mat2, Mat3, Mat4, Object, Sample, Sampler2d, Scalar, Vec2, Vec3, Vec4},
    Logical,
};

use super::{
    Block, FragmentData, FragmentDataVisitor, Primitive, UniformData, VertexData, VertexDataVisitor,
};

// Block

#[sealed]
impl super::BlockView for Logical {
    type Bool = Scalar<bool>;
    type F32 = Scalar<f32>;
    type I32 = Scalar<i32>;
    type U32 = Scalar<u32>;
    type Vec2 = Vec2<f32>;
    type Vec3 = Vec3<f32>;
    type Vec4 = Vec4<f32>;
    type Mat2 = Mat2;
    type Mat3 = Mat3;
    type Mat4 = Mat4;
}

unsafe impl<T: Primitive> Block<Logical> for Scalar<T> {
    type Physical = T;
    type Logical = Self;

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
        unsafe impl<T: Primitive> Block<Logical> for $ty<T> {
            type Logical = Self;
            type Physical = T::$ty;

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
        unsafe impl Block<Logical> for $ty {
            type Logical = Self;
            type Physical = mint::$mint_ty<f32>;

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

// VertexData

#[sealed]
impl super::VertexDataView for Logical {
    type Block<B: Block<Logical>> = B;
}

unsafe impl<B: Block<Logical>> VertexData<Logical> for B {
    type Physical = gl::VertexBuffer<B>;
    type Logical = B::Logical;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexDataVisitor<'a, Logical>) {
        visitor.accept(path, VertexInputRate::Vertex, self);
    }

    fn shader_input(path: &str) -> Self {
        B::vertex_input(path)
    }
}

#[sealed]
impl<B: Block<Logical>> super::VertexDataField<Logical> for B {
    fn shader_input(path: &str) -> Self {
        B::vertex_input(path)
    }
}

// UniformData

#[sealed]
impl super::UniformDataView for Logical {
    type Block<B: Block<Logical, Logical = B>> = B;
    type Sampler2d<S: Sample> = Sampler2d<S>;
    type Compose<R: UniformData<Logical>> = R;
}

unsafe impl<B: Block<Logical, Logical = B>> UniformData<Logical> for B {
    type Logical = Self;
    type Physical = gl::UniformBufferBinding<B>;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformDataVisitor<'a, Logical>) {
        visitor.accept_block(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <B as Block<Logical>>::uniform_input(path)
    }
}

unsafe impl<S: Sample> UniformData<Logical> for Sampler2d<S> {
    type Logical = Self;
    type Physical = gl::Sampler2d<S>;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformDataVisitor<'a, Logical>) {
        visitor.accept_sampler2d(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <Self as Object>::from_arg(path)
    }
}

// FragmentData

#[sealed]
impl super::FragmentDataView for Logical {
    type Attachment = Vec4<f32>;
}

unsafe impl FragmentData<Logical> for Vec4<f32> {
    type Logical = Self;
    type Physical = gl::Texture2d<gl::RgbaFormat>;

    fn visit(&self, path: &str, visitor: &mut impl FragmentDataVisitor<Logical>) {
        visitor.accept(path, self);
    }
}
