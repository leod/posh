use sealed::sealed;

use crate::{
    gl::{RgbaFormat, Sampler2d, Texture2d, UniformBufferBinding, VertexBuffer},
    program_def::VertexInputRate,
    sl, Logical, Physical,
};

use super::{
    Block, FragmentData, FragmentDataVisitor, Primitive, UniformData, VertexData, VertexDataVisitor,
};

// Block

#[sealed]
impl super::BlockView for Physical {
    type Bool = bool;
    type F32 = f32;
    type I32 = i32;
    type U32 = u32;
    type Vec2 = mint::Vector2<f32>;
    type Vec3 = mint::Vector3<f32>;
    type Vec4 = mint::Vector4<f32>;
    type Mat2 = mint::ColumnMatrix2<f32>;
    type Mat3 = mint::ColumnMatrix3<f32>;
    type Mat4 = mint::ColumnMatrix4<f32>;
}

unsafe impl Block<Physical> for bool {
    type Physical = Self;
    type Logical = sl::Scalar<Self>;
}

unsafe impl Block<Physical> for f32 {
    type Physical = Self;
    type Logical = sl::Scalar<Self>;
}

unsafe impl Block<Physical> for i32 {
    type Physical = Self;
    type Logical = sl::Scalar<Self>;
}

unsafe impl Block<Physical> for u32 {
    type Physical = Self;
    type Logical = sl::Scalar<Self>;
}

unsafe impl<T: Primitive> Block<Physical> for mint::Vector2<T> {
    type Physical = T::Vec2;
    type Logical = sl::Vec2<T>;
}

unsafe impl<T: Primitive> Block<Physical> for mint::Vector3<T> {
    type Physical = T::Vec3;
    type Logical = sl::Vec3<T>;
}

unsafe impl<T: Primitive> Block<Physical> for mint::Vector4<T> {
    type Physical = T::Vec4;
    type Logical = sl::Vec4<T>;
}

unsafe impl Block<Physical> for mint::ColumnMatrix2<f32> {
    type Physical = Self;
    type Logical = sl::Mat2;
}

unsafe impl Block<Physical> for mint::ColumnMatrix3<f32> {
    type Physical = Self;
    type Logical = sl::Mat3;
}

unsafe impl Block<Physical> for mint::ColumnMatrix4<f32> {
    type Physical = Self;
    type Logical = sl::Mat4;
}

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
    type Sampler2d<S: sl::Sample> = Sampler2d<S>;
    type Compose<R: UniformData<Logical>> = R::Physical;
}

unsafe impl<U: Block<Logical, Logical = U>> UniformData<Physical> for UniformBufferBinding<U> {
    type Physical = Self;
    type Logical = U;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformDataVisitor<'a, Physical>) {
        visitor.accept_block::<U::Logical>(path, self);
    }
}

unsafe impl<S: sl::Sample> UniformData<Physical> for Sampler2d<S> {
    type Physical = Self;
    type Logical = sl::Sampler2d<S>;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformDataVisitor<'a, Physical>) {
        visitor.accept_sampler2d(path, self);
    }
}

// FragmentData

#[sealed]
impl super::FragmentDataView for Physical {
    type Attachment = Texture2d<RgbaFormat>;
}

unsafe impl FragmentData<Physical> for Texture2d<RgbaFormat> {
    type Physical = Self;
    type Logical = sl::Vec4<f32>;

    fn visit(&self, path: &str, visitor: &mut impl FragmentDataVisitor<Physical>) {
        visitor.accept(path, self);
    }
}
