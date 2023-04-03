use sealed::sealed;

use crate::{
    gl,
    internal::join_ident_path,
    sl::{self, dag::BuiltInType, program_def::VertexAttributeDef, ColorSample, Object},
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

macro_rules! impl_block_for_scalar {
    ($scalar:ident) => {
        unsafe impl Block<Sl> for sl::$scalar {
            type Gl = sl::scalar_physical!($scalar);
            type Sl = Self;

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
        unsafe impl Block<Sl> for sl::$vec {
            type Sl = Self;
            type Gl = glam::$vec;

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
        unsafe impl Block<Sl> for sl::$mat {
            type Sl = Self;
            type Gl = glam::$mat;

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

    fn shader_input(_: &str) -> () {
        ()
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
