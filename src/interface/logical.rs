use sealed::sealed;

use crate::{
    gl,
    sl::{
        self,
        dag::BuiltInType,
        program_def::{VertexAttributeDef, VertexInputRate},
        Object,
    },
    Logical,
};

use super::{Block, FragmentData, FragmentDataVisitor, UniformData, VertexData, VertexDataVisitor};

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
    type Sampler2d = sl::Sampler2d;
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

unsafe impl UniformData<Logical> for sl::Sampler2d {
    type Logical = Self;
    type Physical = gl::Sampler2d;

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
    type Attachment = sl::Vec4;
}

unsafe impl FragmentData<Logical> for sl::Vec4 {
    type Logical = Self;
    type Physical = gl::Texture2d<gl::RgbaFormat>;

    fn visit(&self, path: &str, visitor: &mut impl FragmentDataVisitor<Logical>) {
        visitor.accept(path, self);
    }
}
