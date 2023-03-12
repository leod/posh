use sealed::sealed;

use crate::{
    gl::{Attachment, Sampler2d, UniformBufferBinding, VertexBufferBinding},
    internal::join_ident_path,
    sl::{self, program_def::VertexInputRate, ColorSample},
};

use super::{Block, Fragment, FragmentVisitor, GlView, SlView, Uniform, Vertex, VertexVisitor};

// Block

#[sealed]
impl super::BlockFields for GlView {
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
        unsafe impl Block<GlView> for sl::scalar_physical!($scalar) {
            type GlView = Self;
            type SlView = sl::$scalar;
        }
    };
}

macro_rules! impl_block_for_vec {
    ($vec:ident) => {
        unsafe impl Block<GlView> for glam::$vec {
            type GlView = Self;
            type SlView = sl::$vec;
        }
    };
}

macro_rules! impl_block_for_mat {
    ($mat:ident) => {
        unsafe impl Block<GlView> for glam::$mat {
            type GlView = Self;
            type SlView = sl::$mat;
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

// Vertex

#[sealed]
impl super::VertexFields for GlView {
    type Block<B: Block<SlView>> = VertexBufferBinding<B>;
}

unsafe impl<B: Block<SlView>> Vertex<GlView> for VertexBufferBinding<B> {
    type GlView = Self;
    type SlView = B::SlView;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexVisitor<'a, GlView>) {
        visitor.accept(path, VertexInputRate::Vertex, self)
    }
}

#[sealed]
impl<B: Block<SlView>> super::VertexField<GlView> for VertexBufferBinding<B> {}

// Uniform

#[sealed]
impl super::UniformFields for GlView {
    type Block<B: Block<SlView, SlView = B>> = UniformBufferBinding<B>;
    type Sampler2d<S: ColorSample> = Sampler2d<S>;
    type Compose<R: Uniform<SlView>> = R::GlView;
}

unsafe impl<U: Block<SlView, SlView = U>> Uniform<GlView> for UniformBufferBinding<U> {
    type GlView = Self;
    type SlView = U;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformVisitor<'a, GlView>) {
        visitor.accept_block::<U::SlView>(path, self);
    }
}

unsafe impl<S: ColorSample> Uniform<GlView> for Sampler2d<S> {
    type GlView = Self;
    type SlView = sl::Sampler2d<S>;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformVisitor<'a, GlView>) {
        visitor.accept_sampler2d(path, self);
    }
}

unsafe impl<U, V> Uniform<GlView> for (U, V)
where
    U: Uniform<GlView>,
    V: Uniform<GlView>,
{
    type SlView = (U::SlView, V::SlView);
    type GlView = Self;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformVisitor<'a, GlView>) {
        self.0.visit(&join_ident_path(path, "a"), visitor);
        self.1.visit(&join_ident_path(path, "b"), visitor);
    }
}

// Fragment

#[sealed]
impl super::FragmentFields for GlView {
    type Attachment2d<S: ColorSample> = Attachment<S>;
}

unsafe impl<S: ColorSample> Fragment<GlView> for Attachment<S> {
    type GlView = Self;
    type SlView = S;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl FragmentVisitor<'a, GlView>) {
        visitor.accept(path, self);
    }
}
