use sealed::sealed;

use crate::{
    gl::{
        self, ColorAttachment, ColorSampler2d, ComparisonSampler2d, UniformBufferBinding,
        VertexBufferBinding,
    },
    internal::join_ident_path,
    sl::{self, program_def::VertexAttributeDef, ColorSample},
};

use super::{Block, Fragment, FragmentVisitor, Gl, Sl, Uniform, Vertex, VertexVisitor};

// Block

#[sealed]
impl super::BlockDom for Gl {
    type F32 = f32;
    type I32 = i32;
    type U32 = u32;
    type Vec2 = gl::Vec2;
    type Vec3 = gl::Vec3;
    type Vec4 = gl::Vec4;
    type IVec2 = gl::IVec2;
    type IVec3 = gl::IVec3;
    type IVec4 = gl::IVec4;
    type UVec2 = gl::UVec2;
    type UVec3 = gl::UVec3;
    type UVec4 = gl::UVec4;
    type Mat2 = gl::Mat2;
    type Mat3 = gl::Mat3;
    type Mat4 = gl::Mat4;
}

macro_rules! impl_block {
    ($gl:ty, $sl:ty) => {
        unsafe impl Block<Gl> for $gl {
            type Gl = $gl;
            type Sl = $sl;
        }

        unsafe impl Block<Sl> for $sl {
            type Gl = $gl;
            type Sl = $sl;

            fn uniform_input(path: &str) -> Self {
                <Self as sl::Object>::from_arg(path)
            }

            fn vertex_input(path: &str) -> Self {
                // FIXME: Cast from u32 to bool!
                <Self as sl::Object>::from_arg(path)
            }

            fn vertex_attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
                vec![VertexAttributeDef {
                    name: path.to_string(),
                    ty: <Self as sl::Object>::ty().built_in_type().unwrap(),
                    offset: 0,
                }]
            }
        }
    };
}

impl_block!(f32, sl::F32);
impl_block!(i32, sl::I32);
impl_block!(u32, sl::U32);
impl_block!(gl::Vec2, sl::Vec2);
impl_block!(gl::Vec3, sl::Vec3);
impl_block!(gl::Vec4, sl::Vec4);
impl_block!(gl::IVec2, sl::IVec2);
impl_block!(gl::IVec3, sl::IVec3);
impl_block!(gl::IVec4, sl::IVec4);
impl_block!(gl::UVec2, sl::UVec2);
impl_block!(gl::UVec3, sl::UVec3);
impl_block!(gl::UVec4, sl::UVec4);
impl_block!(gl::Mat2, sl::Mat2);
impl_block!(gl::Mat3, sl::Mat3);
impl_block!(gl::Mat4, sl::Mat4);

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

unsafe impl<U, V> Vertex<Gl> for (U, V)
where
    U: Vertex<Gl>,
    V: Vertex<Gl>,
{
    type Sl = (U::Sl, V::Sl);
    type Gl = Self;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::VertexVisitor<'a, Gl>) {
        self.0.visit(&join_ident_path(path, "a"), visitor);
        self.1.visit(&join_ident_path(path, "b"), visitor);
    }
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
