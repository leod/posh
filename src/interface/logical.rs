use sealed::sealed;

use crate::{
    gl,
    internal::join_ident_path,
    sl::{
        self,
        dag::BuiltInType,
        program_def::{VertexAttributeDef, VertexInputRate},
        Object, Sample,
    },
};

use super::{
    Block, FragmentData, FragmentDataVisitor, Logical, UniformData, UniformDataNonUnit, VertexData,
    VertexDataVisitor,
};

// Block

#[sealed]
impl super::BlockView for Logical {
    type F32 = sl::F32;
    type I32 = sl::I32;
    type U32 = sl::U32;
    type Bool = sl::Bool;
    type Vec2 = sl::Vec2;
    type Vec3 = sl::Vec3;
    type Vec4 = sl::Vec4;
    type Mat2 = sl::Mat2;
    type Mat3 = sl::Mat3;
    type Mat4 = sl::Mat4;
}

macro_rules! impl_block_for_scalar {
    ($scalar:ident) => {
        unsafe impl Block<Logical> for sl::$scalar {
            type Physical = sl::scalar_physical!($scalar);
            type Logical = Self;

            fn uniform_input(path: &str) -> Self {
                <Self as Object>::from_arg(path)
            }

            fn vertex_input(path: &str) -> Self {
                // FIXME: Cast from u32 to bool!
                <Self as Object>::from_arg(path)
            }

            fn vertex_attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
                vec![VertexAttributeDef {
                    name: path.to_string(),
                    ty: BuiltInType::$scalar,
                    offset: 0,
                }]
            }
        }
    };
}

macro_rules! impl_block_for_vec {
    ($vec:ident) => {
        unsafe impl Block<Logical> for sl::$vec {
            type Logical = Self;
            type Physical = glam::$vec;

            fn uniform_input(path: &str) -> Self {
                <Self as Object>::from_arg(path)
            }

            fn vertex_input(path: &str) -> Self {
                // FIXME: Cast from u32 to bool!
                <Self as Object>::from_arg(path)
            }

            fn vertex_attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
                vec![VertexAttributeDef {
                    name: path.to_string(),
                    ty: <Self as Object>::ty().built_in_type().unwrap(),
                    offset: 0,
                }]
            }
        }
    };
}

macro_rules! impl_block_for_mat {
    ($mat:ident) => {
        unsafe impl Block<Logical> for sl::$mat {
            type Logical = Self;
            type Physical = glam::$mat;

            fn uniform_input(path: &str) -> Self {
                <Self as Object>::from_arg(path)
            }

            fn vertex_input(path: &str) -> Self {
                <Self as Object>::from_arg(path)
            }

            fn vertex_attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
                vec![VertexAttributeDef {
                    name: path.to_string(),
                    ty: BuiltInType::$mat,
                    offset: 0,
                }]
            }
        }
    };
}

impl_block_for_scalar!(F32);
impl_block_for_scalar!(I32);
impl_block_for_scalar!(U32);
impl_block_for_scalar!(Bool);

impl_block_for_vec!(Vec2);
impl_block_for_vec!(IVec2);
impl_block_for_vec!(UVec2);
impl_block_for_vec!(Vec3);
impl_block_for_vec!(IVec3);
impl_block_for_vec!(UVec3);
impl_block_for_vec!(Vec4);
impl_block_for_vec!(IVec4);
impl_block_for_vec!(UVec4);

impl_block_for_mat!(Mat2);
impl_block_for_mat!(Mat3);
impl_block_for_mat!(Mat4);

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
    type Sampler2d<S: Sample> = sl::Sampler2d<S>;
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

impl<B: Block<Logical, Logical = B>> UniformDataNonUnit for B {}

unsafe impl<S: Sample> UniformData<Logical> for sl::Sampler2d<S> {
    type Logical = Self;
    type Physical = gl::Sampler2d<S>;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformDataVisitor<'a, Logical>) {
        visitor.accept_sampler2d(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <Self as Object>::from_arg(path)
    }
}

impl<S: Sample> UniformDataNonUnit for sl::Sampler2d<S> {}

unsafe impl<U, V> UniformData<Logical> for (U, V)
where
    U: UniformData<Logical>,
    V: UniformData<Logical>,
{
    type Logical = Self;
    type Physical = (U::Physical, V::Physical);

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformDataVisitor<'a, Logical>) {
        self.0.visit(&join_ident_path(path, "a"), visitor);
        self.1.visit(&join_ident_path(path, "b"), visitor);
    }

    fn shader_input(path: &str) -> Self {
        (
            U::shader_input(&join_ident_path(path, "a")),
            V::shader_input(&join_ident_path(path, "b")),
        )
    }
}

impl<U, V> UniformDataNonUnit for (U, V)
where
    U: UniformData<Logical>,
    V: UniformData<Logical>,
{
}

// FragmentData

#[sealed]
impl super::FragmentDataView for Logical {
    type Attachment = sl::Vec4;
}

unsafe impl FragmentData<Logical> for sl::Vec4 {
    type Logical = Self;
    type Physical = gl::Texture2d<sl::Vec4>;

    fn visit(&self, path: &str, visitor: &mut impl FragmentDataVisitor<Logical>) {
        visitor.accept(path, self);
    }
}
