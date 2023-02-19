use sealed::sealed;

use crate::{
    gl::{Sampler2d, Texture2d, UniformBufferBinding, VertexBuffer},
    internal::join_ident_path,
    sl::{self, program_def::VertexInputRate, Sample},
};

use super::{
    Block, FragmentData, FragmentDataVisitor, Logical, Physical, UniformData, VertexData,
    VertexDataVisitor,
};

// Block

#[sealed]
impl super::BlockView for Physical {
    type Bool = bool;
    type F32 = f32;
    type I32 = i32;
    type U32 = u32;
    type Vec2 = glam::Vec2;
    type Vec3 = glam::Vec3;
    type Vec4 = glam::Vec4;
    type Mat2 = glam::Mat2;
    type Mat3 = glam::Mat3;
    type Mat4 = glam::Mat4;
}

macro_rules! impl_block_for_scalar {
    ($scalar:ident) => {
        unsafe impl Block<Physical> for sl::scalar_physical!($scalar) {
            type Physical = Self;
            type Logical = sl::$scalar;
        }
    };
}

macro_rules! impl_block_for_vec {
    ($vec:ident) => {
        unsafe impl Block<Physical> for glam::$vec {
            type Physical = Self;
            type Logical = sl::$vec;
        }
    };
}

macro_rules! impl_block_for_mat {
    ($mat:ident) => {
        unsafe impl Block<Physical> for glam::$mat {
            type Physical = Self;
            type Logical = sl::$mat;
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
impl super::VertexDataView for Physical {
    type Block<B: Block<Logical>> = VertexBuffer<B>;
}

unsafe impl<B: Block<Logical>> VertexData<Physical> for VertexBuffer<B> {
    type Physical = Self;
    type Logical = B::Logical;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexDataVisitor<'a, Physical>) {
        visitor.accept(path, VertexInputRate::Vertex, self)
    }
}

#[sealed]
impl<B: Block<Logical>> super::VertexDataField<Physical> for VertexBuffer<B> {}

// UniformData

#[sealed]
impl super::UniformDataView for Physical {
    type Block<B: Block<Logical, Logical = B>> = UniformBufferBinding<B>;
    type Sampler2d<S: Sample> = Sampler2d<S>;
    type Compose<R: UniformData<Logical>> = R::Physical;
}

unsafe impl<U: Block<Logical, Logical = U>> UniformData<Physical> for UniformBufferBinding<U> {
    type Physical = Self;
    type Logical = U;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformDataVisitor<'a, Physical>) {
        visitor.accept_block::<U::Logical>(path, self);
    }
}

unsafe impl<S: Sample> UniformData<Physical> for Sampler2d<S> {
    type Physical = Self;
    type Logical = sl::Sampler2d<S>;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformDataVisitor<'a, Physical>) {
        visitor.accept_sampler2d(path, self);
    }
}

unsafe impl<U, V> UniformData<Physical> for (U, V)
where
    U: UniformData<Physical>,
    V: UniformData<Physical>,
{
    type Logical = (U::Logical, V::Logical);
    type Physical = Self;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformDataVisitor<'a, Physical>) {
        self.0.visit(&join_ident_path(path, "a"), visitor);
        self.1.visit(&join_ident_path(path, "b"), visitor);
    }
}

// FragmentData

#[sealed]
impl super::FragmentDataView for Physical {
    type Attachment = Texture2d<sl::Vec4>;
}

unsafe impl FragmentData<Physical> for Texture2d<sl::Vec4> {
    type Physical = Self;
    type Logical = sl::Vec4;

    fn visit(&self, path: &str, visitor: &mut impl FragmentDataVisitor<Physical>) {
        visitor.accept(path, self);
    }
}
