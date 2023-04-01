use sealed::sealed;

use crate::{
    gl::{
        ColorAttachment, ColorSampler2d, ComparisonSampler2d, UniformBufferBinding,
        VertexBufferBinding,
    },
    internal::join_ident_path,
    sl::{self, ColorSample},
};

use super::{Block, Fragment, FragmentVisitor, Gl, Sl, Uniform, Vertex, VertexVisitor};

// Block

#[sealed]
impl super::BlockDom for Gl {
    type F32 = f32;
    type I32 = i32;
    type U32 = u32;
    type Vec2 = glam::Vec2;
    type Vec3 = glam::Vec3;
    type Vec4 = glam::Vec4;
    type IVec2 = glam::IVec2;
    type IVec3 = glam::IVec3;
    type IVec4 = glam::IVec4;
    type UVec2 = glam::UVec2;
    type UVec3 = glam::UVec3;
    type UVec4 = glam::UVec4;
    type Mat2 = glam::Mat2;
    type Mat3 = glam::Mat3;
    type Mat4 = glam::Mat4;
}

macro_rules! impl_block_for_scalar {
    ($scalar:ident) => {
        unsafe impl Block<Gl> for sl::scalar_physical!($scalar) {
            type Gl = Self;
            type Sl = sl::$scalar;
        }
    };
}

macro_rules! impl_block_for_vec {
    ($vec:ident) => {
        unsafe impl Block<Gl> for glam::$vec {
            type Gl = Self;
            type Sl = sl::$vec;
        }
    };
}

macro_rules! impl_block_for_mat {
    ($mat:ident) => {
        unsafe impl Block<Gl> for glam::$mat {
            type Gl = Self;
            type Sl = sl::$mat;
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
impl super::VertexDom for Gl {
    type Block<B: Block<Sl>> = VertexBufferBinding<B>;
}

unsafe impl<B: Block<Sl>> Vertex<Gl> for VertexBufferBinding<B> {
    type Gl = Self;
    type Sl = B::Sl;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexVisitor<'a, Gl>) {
        visitor.accept(path, self)
    }
}

unsafe impl Vertex<Gl> for () {
    type Gl = ();
    type Sl = ();

    fn visit<'a>(&'a self, _: &str, _: &mut impl VertexVisitor<'a, Gl>) {}
}

#[sealed]
impl<B: Block<Sl>> super::VertexField<Gl> for VertexBufferBinding<B> {}

// Uniform

#[sealed]
impl super::UniformDom for Gl {
    type Block<B: Block<Sl, Sl = B>> = UniformBufferBinding<B>;
    type ColorSampler2d<S: ColorSample> = ColorSampler2d<S>;
    type ComparisonSampler2d = ComparisonSampler2d;
    type Uniform<R: Uniform<Sl>> = R::Gl;
}

unsafe impl<U: Block<Sl, Sl = U>> Uniform<Gl> for UniformBufferBinding<U> {
    type Gl = Self;
    type Sl = U;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformVisitor<'a, Gl>) {
        visitor.accept_block::<U::Sl>(path, self);
    }
}

unsafe impl<S: ColorSample> Uniform<Gl> for ColorSampler2d<S> {
    type Gl = Self;
    type Sl = sl::ColorSampler2d<S>;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformVisitor<'a, Gl>) {
        visitor.accept_color_sampler_2d(path, self);
    }
}

unsafe impl Uniform<Gl> for ComparisonSampler2d {
    type Gl = Self;
    type Sl = sl::ComparisonSampler2d;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformVisitor<'a, Gl>) {
        visitor.accept_comparison_sampler_2d(path, self);
    }
}

unsafe impl<U, V> Uniform<Gl> for (U, V)
where
    U: Uniform<Gl>,
    V: Uniform<Gl>,
{
    type Sl = (U::Sl, V::Sl);
    type Gl = Self;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformVisitor<'a, Gl>) {
        self.0.visit(&join_ident_path(path, "a"), visitor);
        self.1.visit(&join_ident_path(path, "b"), visitor);
    }
}

// Fragment

#[sealed]
impl super::FragmentDom for Gl {
    type Output<S: ColorSample> = ColorAttachment<S>;
}

unsafe impl<S: ColorSample> Fragment<Gl> for ColorAttachment<S> {
    type Gl = Self;
    type Sl = S;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl FragmentVisitor<'a, Gl>) {
        visitor.accept(path, self);
    }
}

unsafe impl Fragment<Gl> for () {
    type Sl = ();
    type Gl = ();

    fn visit<'a>(&'a self, _: &str, _: &mut impl FragmentVisitor<'a, Gl>) {}
}
