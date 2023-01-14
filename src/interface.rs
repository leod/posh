use bytemuck::Pod;
use crevice::std140::AsStd140;
use sealed::sealed;

mod gl;
mod sl;

use crate::{
    program_def::{VertexAttributeDef, VertexInputRate},
    sl::{Bool, Scalar, ToValue, Value, Vec2, F32, I32, U32},
    Gl, Numeric, Primitive, Sl,
};

/// Provides types for [`Uniform`] or [`Vertex`] declarations.
#[sealed]
pub trait Domain: Copy {
    /// A scalar value.
    type Scalar<T: Primitive>: Uniform<Self> + Vertex<Self> + ToValue<Output = Scalar<T>>;

    /// A two-dimensional vector.
    type Vec2<T: Primitive>: Uniform<Self> + Vertex<Self> + ToValue<Output = Vec2<T>>;

    /// A boolean value.
    ///
    /// Shorthand for [`Self::Scalar<bool>`].
    type Bool: Uniform<Self> + Vertex<Self> + ToValue<Output = Bool>;

    /// A floating-point value.
    ///
    /// Shorthand for [`Self::Scalar<f32>`].
    type F32: Uniform<Self> + Vertex<Self> + ToValue<Output = F32>;

    /// A signed integer value.
    ///
    /// Shorthand for [`Self::Scalar<i32>`].
    type I32: Uniform<Self> + Vertex<Self> + ToValue<Output = I32>;

    /// An unsigned integer value.
    ///
    /// Shorthand for [`Self::Scalar<u32>`].
    type U32: Uniform<Self> + Vertex<Self> + ToValue<Output = U32>;
}

// Uniform interface

/// Uniform types.
pub trait Uniform<D: Domain>: ToValue {
    /// The representation of [`Self`] in the graphics library domain [`Gl`].
    ///
    /// This is the type through which uniform data is provided on the host.
    type InGl: Uniform<Gl> + AsStd140 + ToValue<Output = Self::InSl>;

    /// The representation of [`Self`] in the shading language domain [`Sl`].
    ///
    /// This is the type through which uniform data is read in shaders.
    type InSl: Uniform<Sl> + Value + ToValue<Output = Self::InSl>;

    #[doc(hidden)]
    fn shader_input(path: &str) -> Self;
}

// Vertex

/// A conversion to a type that implements [`Pod`].
///
/// TODO: This is a workarond for `mint` not supporting `bytemuck`.
pub trait ToPod: Copy {
    type Output: Pod;

    fn to_pod(self) -> Self::Output;
}

/// Vertex types.
pub trait Vertex<D: Domain>: ToValue {
    /// The representation of [`Self`] in the graphics library domain [`Gl`].
    ///
    /// This is the type through which vertex data is provided on the host.
    type InGl: Vertex<Gl> + ToPod<Output = Self::Pod> + ToValue<Output = Self::InSl>;

    /// The representation of [`Self`] in the shading language domain [`Sl`].
    ///
    /// This is the type through which vertex data is read in shaders.
    type InSl: Vertex<Sl> + Value + ToValue<Output = Self::InSl>;

    // TODO: This is a workarond for `mint` not supporting `bytemuck`.
    #[doc(hidden)]
    type Pod: Pod;

    #[doc(hidden)]
    fn attribute_defs(path: &str) -> Vec<VertexAttributeDef>;

    #[doc(hidden)]
    fn shader_input(path: &str) -> Self;
}

// VertexInterface

/// Provides types for [`VertexInterface`] declarations.
#[sealed]
pub trait VertexDomain: Domain {
    /// A vertex field.
    type Vertex<V: Vertex<Sl>>: VertexInterfaceField<Self>;
}

/// Types that can occur in a [`VertexInterface`].
#[sealed]
#[doc(hidden)]
pub trait VertexInterfaceField<D: VertexDomain> {
    fn shader_input(path: &str) -> Self;
}

/// A vertex input interface for shaders.
pub trait VertexInterface<D: VertexDomain> {
    /// The representation of [`Self`] in the graphics library domain [`Gl`].
    ///
    /// This is the type through which vertex buffers are bound on the host.
    type InGl: VertexInterface<Gl>;

    /// The representation of [`Self`] in the shading language domain [`Sl`].
    ///
    /// This is the type through which the vertex interface is accessed in shaders.
    type InSl: VertexInterface<Sl>;

    #[doc(hidden)]
    fn visit(&self, path: &str, visitor: &mut impl VertexInterfaceVisitor<D>);

    #[doc(hidden)]
    fn shader_input(path: &str) -> Self;
}

pub trait VertexInterfaceVisitor<D: VertexDomain> {
    fn accept<V: Vertex<Sl>>(
        &mut self,
        path: &str,
        input_rate: VertexInputRate,
        vertex: &D::Vertex<V>,
    );
}

// ResourceInterface

/// Provides types for [`ResourceInterface`] declarations.
#[sealed]
pub trait ResourceDomain: Domain {
    /// A two-dimensional sampler field.
    type Sampler2d<T: Numeric>: ResourceInterface<Self>;

    /// A uniform field.
    type Uniform<U: Uniform<Sl, InSl = U>>: ResourceInterface<Self>;

    /// A resource interface field.
    type Compose<R: ResourceInterface<Sl>>: ResourceInterface<Self>;
}

/// A shader resource interface.
///
/// Resources provide access to uniforms or samplers to shaders. Resources can
/// be used in both vertex shaders and fragment shaders. In order to link a
/// vertex shader with a fragment shader, they must use the same resource type.
/// See [`create_program`](crate::gl::Context::create_program) for details.
pub trait ResourceInterface<D: ResourceDomain> {
    /// The representation of [`Self`] in the graphics library domain [`Gl`].
    ///
    /// This type provides resource bindings such as [uniform
    /// buffers](crate::gl::UniformBuffer) on the host.
    type InGl: ResourceInterface<Gl>;

    /// The representation of [`Self`] in the shading language domain [`Sl`].
    ///
    /// T
    type InSl: ResourceInterface<Sl>;

    #[doc(hidden)]
    fn visit(&self, path: &str, visitor: &mut impl ResourceInterfaceVisitor<D>);

    #[doc(hidden)]
    fn shader_input(path: &str) -> Self;
}

impl<D: ResourceDomain> ResourceInterface<D> for () {
    type InGl = ();
    type InSl = ();

    fn visit(&self, _: &str, _: &mut impl ResourceInterfaceVisitor<D>) {}

    fn shader_input(_: &str) -> Self {}
}

#[doc(hidden)]
pub trait ResourceInterfaceVisitor<D: ResourceDomain> {
    fn accept_sampler2d<T: Numeric>(&mut self, path: &str, sampler: &D::Sampler2d<T>);

    fn accept_uniform<U: Uniform<Sl, InSl = U>>(&mut self, path: &str, uniform: &D::Uniform<U>);
}

// FragmentInterface

/// Provides types for [`FragmentInterface`] declarations.
#[sealed]
pub trait FragmentDomain: Sized {
    type Attachment: FragmentInterface<Self>;
}

/// A fragment shader output interface.
pub trait FragmentInterface<D: FragmentDomain> {
    /// The representation of [`Self`] in the graphics library domain [`Gl`].
    ///
    /// Provides framebuffer attachments on the host.
    type InGl: FragmentInterface<Gl>;

    /// The representation of [`Self`] in the shading language domain [`Sl`].
    ///
    /// Contains expressions representing fragment shader output.
    type InSl: FragmentInterface<Sl>;

    #[doc(hidden)]
    fn visit(&self, path: &str, visitor: &mut impl FragmentInterfaceVisitor<D>);
}

#[doc(hidden)]
pub trait FragmentInterfaceVisitor<D: FragmentDomain> {
    fn accept(&mut self, path: &str, attachment: &D::Attachment);
}
