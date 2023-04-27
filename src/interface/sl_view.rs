use sealed::sealed;

use crate::{
    gl,
    internal::join_ident_path,
    sl::{self, ColorSample, Object},
};

use super::{Block, Fragment, FragmentVisitor, Sl, Uniform, UniformNonUnit, Vertex, VertexVisitor};

// Block

#[sealed]
impl super::BlockDom for Sl {
    type F32 = sl::F32;
    type I32 = sl::I32;
    type U32 = sl::U32;
    type Vec2 = sl::Vec2;
    type Vec3 = sl::Vec3;
    type Vec4 = sl::Vec4;
    type IVec2 = sl::IVec2;
    type IVec3 = sl::IVec3;
    type IVec4 = sl::IVec4;
    type UVec2 = sl::UVec2;
    type UVec3 = sl::UVec3;
    type UVec4 = sl::UVec4;
    type Mat2 = sl::Mat2;
    type Mat3 = sl::Mat3;
    type Mat4 = sl::Mat4;
}

// Vertex

#[sealed]
impl super::VertexDom for Sl {
    type Block<B: Block<Sl>> = B;
}

unsafe impl<B: Block<Sl>> Vertex<Sl> for B {
    type Gl = gl::VertexBufferBinding<B>;
    type Sl = B::Sl;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexVisitor<'a, Sl>) {
        visitor.accept(path, self);
    }

    fn shader_input(path: &str) -> Self {
        B::vertex_input(path)
    }
}

unsafe impl Vertex<Sl> for () {
    type Gl = ();
    type Sl = ();

    fn visit<'a>(&'a self, _: &str, _: &mut impl VertexVisitor<'a, Sl>) {}

    fn shader_input(_: &str) {}
}

unsafe impl<U, V> Vertex<Sl> for (U, V)
where
    U: Vertex<Sl>,
    V: Vertex<Sl>,
{
    type Sl = Self;
    type Gl = (U::Gl, V::Gl);

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::VertexVisitor<'a, Sl>) {
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

#[sealed]
impl<B: Block<Sl>> super::VertexField<Sl> for B {
    fn shader_input(path: &str) -> Self {
        B::vertex_input(path)
    }
}

// Uniform

#[sealed]
impl super::UniformDom for Sl {
    type Block<B: Block<Sl, Sl = B>> = B;
    type ColorSampler2d<S: ColorSample> = sl::ColorSampler2d<S>;
    type ComparisonSampler2d = sl::ComparisonSampler2d;
    type Uniform<R: Uniform<Sl>> = R;
}

unsafe impl<B: Block<Sl, Sl = B>> Uniform<Sl> for B {
    type Sl = Self;
    type Gl = gl::UniformBufferBinding<B>;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformVisitor<'a, Sl>) {
        visitor.accept_block(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <B as Block<Sl>>::uniform_input(path)
    }
}

impl<B: Block<Sl, Sl = B>> UniformNonUnit for B {}

unsafe impl<S: ColorSample> Uniform<Sl> for sl::ColorSampler2d<S> {
    type Sl = Self;
    type Gl = gl::ColorSampler2d<S>;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformVisitor<'a, Sl>) {
        visitor.accept_color_sampler_2d(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <Self as Object>::from_arg(path)
    }
}

impl<S: ColorSample> UniformNonUnit for sl::ColorSampler2d<S> {}

unsafe impl Uniform<Sl> for sl::ComparisonSampler2d {
    type Sl = Self;
    type Gl = gl::ComparisonSampler2d;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformVisitor<'a, Sl>) {
        visitor.accept_comparison_sampler_2d(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <Self as Object>::from_arg(path)
    }
}

impl UniformNonUnit for sl::ComparisonSampler2d {}

unsafe impl<U, V> Uniform<Sl> for (U, V)
where
    U: Uniform<Sl>,
    V: Uniform<Sl>,
{
    type Sl = Self;
    type Gl = (U::Gl, V::Gl);

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformVisitor<'a, Sl>) {
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

impl<U, V> UniformNonUnit for (U, V)
where
    U: Uniform<Sl>,
    V: Uniform<Sl>,
{
}

// Fragment

#[sealed]
impl super::FragmentDom for Sl {
    type Output<S: ColorSample> = S;
}

unsafe impl<S: ColorSample> Fragment<Sl> for S {
    type Sl = Self;
    type Gl = gl::ColorAttachment<S>;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl FragmentVisitor<'a, Sl>) {
        visitor.accept(path, self);
    }
}

unsafe impl Fragment<Sl> for () {
    type Sl = ();
    type Gl = ();

    fn visit<'a>(&'a self, _: &str, _: &mut impl FragmentVisitor<'a, Sl>) {}
}
