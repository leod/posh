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
    Block, Fragment, FragmentVisitor, SlView, Uniform, UniformNonUnit, Vertex, VertexVisitor,
};

// Block

#[sealed]
impl super::BlockFields for SlView {
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
        unsafe impl Block<SlView> for sl::$scalar {
            type GlView = sl::scalar_physical!($scalar);
            type SlView = Self;

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
        unsafe impl Block<SlView> for sl::$vec {
            type SlView = Self;
            type GlView = glam::$vec;

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
        unsafe impl Block<SlView> for sl::$mat {
            type SlView = Self;
            type GlView = glam::$mat;

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
impl super::VertexFields for SlView {
    type Block<B: Block<SlView>> = B;
}

unsafe impl<B: Block<SlView>> Vertex<SlView> for B {
    type GlView = gl::VertexBufferBinding<B>;
    type SlView = B::SlView;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexVisitor<'a, SlView>) {
        visitor.accept(path, VertexInputRate::Vertex, self);
    }

    fn shader_input(path: &str) -> Self {
        B::vertex_input(path)
    }
}

#[sealed]
impl<B: Block<SlView>> super::VertexField<SlView> for B {
    fn shader_input(path: &str) -> Self {
        B::vertex_input(path)
    }
}

// Uniform

#[sealed]
impl super::UniformFields for SlView {
    type Block<B: Block<SlView, SlView = B>> = B;
    type Sampler2d<S: Sample> = sl::Sampler2d<S>;
    type Compose<R: Uniform<SlView>> = R;
}

unsafe impl<B: Block<SlView, SlView = B>> Uniform<SlView> for B {
    type SlView = Self;
    type GlView = gl::UniformBufferBinding<B>;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformVisitor<'a, SlView>) {
        visitor.accept_block(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <B as Block<SlView>>::uniform_input(path)
    }
}

impl<B: Block<SlView, SlView = B>> UniformNonUnit for B {}

unsafe impl<S: Sample> Uniform<SlView> for sl::Sampler2d<S> {
    type SlView = Self;
    type GlView = gl::Texture2dBinding<S>;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformVisitor<'a, SlView>) {
        visitor.accept_sampler2d(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <Self as Object>::from_arg(path)
    }
}

impl<S: Sample> UniformNonUnit for sl::Sampler2d<S> {}

unsafe impl<U, V> Uniform<SlView> for (U, V)
where
    U: Uniform<SlView>,
    V: Uniform<SlView>,
{
    type SlView = Self;
    type GlView = (U::GlView, V::GlView);

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::UniformVisitor<'a, SlView>) {
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
    U: Uniform<SlView>,
    V: Uniform<SlView>,
{
}

// Fragment

#[sealed]
impl super::FragmentFields for SlView {
    type Attachment = sl::Vec4;
}

unsafe impl Fragment<SlView> for sl::Vec4 {
    type SlView = Self;
    type GlView = gl::Texture2d<sl::Vec4>;

    fn visit(&self, path: &str, visitor: &mut impl FragmentVisitor<SlView>) {
        visitor.accept(path, self);
    }
}
