mod gl;
mod numeric;
mod sl;
#[cfg(tests)]
mod tests;

use crevice::std140::AsStd140;
use sealed::sealed;

use crate::{
    program_def::{VertexAttributeDef, VertexInputRate},
    sl::{Bool, Mat2, Mat3, Mat4, Scalar, ToValue, Value, Vec2, Vec3, Vec4, F32, I32, U32},
    Gl, Sl,
};

pub use numeric::{Numeric, Primitive};

// Block

/// Provides types for [`Block`] declarations.
///
/// See [`Block`] for more details.
#[sealed]
pub trait BlockDomain: Copy {
    /// A scalar value.
    ///
    /// Maps to `T` in the graphics library and [`Scalar<T>`] in the shading language.
    type Scalar<T: Primitive>: Block<Self> + ToValue<Output = Scalar<T>>;

    /// A two-dimensional vector.
    ///
    /// Maps to [`mint::Vector2<T>`] in the graphics library and [`Vec2<T>`] in
    /// the shading language.
    type Vec2<T: Primitive>: Block<Self> + ToValue<Output = Vec2<T>>;

    /// A three-dimensional vector.
    ///
    /// Maps to [`mint::Vector3<T>`] in the graphics library and [`Vec3<T>`] in
    /// the shading language.
    type Vec3<T: Primitive>: Block<Self> + ToValue<Output = Vec3<T>>;

    /// A three-dimensional vector.
    ///
    /// Maps to [`mint::Vector4<T>`] in the graphics library and [`Vec4<T>`] in
    /// the shading language.
    type Vec4<T: Primitive>: Block<Self> + ToValue<Output = Vec4<T>>;

    /// A two-by-two matrix.
    ///
    /// Maps to [`mint::ColumnMatrix2<f32>`] in the graphics library and
    /// [`Mat2`] in the shading language.
    type Mat2: Block<Self> + ToValue<Output = Mat2>;

    /// A three-by-three matrix.
    ///
    /// Maps to [`mint::ColumnMatrix3<f32>`] in the graphics library and
    /// [`Mat3`] in the shading language.
    type Mat3: Block<Self> + ToValue<Output = Mat3>;

    /// A four-by-four matrix.
    ///
    /// Maps to [`mint::ColumnMatrix4<f32>`] in the graphics library and
    /// [`Mat4`] in the shading language.
    type Mat4: Block<Self> + ToValue<Output = Mat4>;

    /// A boolean value.
    ///
    /// Shorthand for [`Self::Scalar<bool>`]. Maps to [`bool`] in the graphics
    /// library and [`Bool`] in the shading language.
    type Bool: Block<Self> + ToValue<Output = Bool>;

    /// A floating-point value.
    ///
    /// Shorthand for [`Self::Scalar<f32>`]. Maps to [`f32`] in the graphics
    /// library and [`F32`] in the shading language.
    type F32: Block<Self> + ToValue<Output = F32>;

    /// A signed integer value.
    ///
    /// Shorthand for [`Self::Scalar<i32>`]. Maps to [`i32`] in the graphics
    /// library and [`I32`] in the shading language.
    type I32: Block<Self> + ToValue<Output = I32>;

    /// An unsigned integer value.
    ///
    /// Shorthand for [`Self::Scalar<u32>`]. Maps to [`u32`] in the graphics
    /// library and [`U32`] in the shading language.
    type U32: Block<Self> + ToValue<Output = U32>;
}

/// Plain-old-data that can be passed to shaders.
///
/// Types that implement [`Block`] can appear in two parts of shader interfaces:
/// 1. As vertex data in a [`VertexInterface`].
/// 2. As uniform data in a [`ResourceInterface`]
///
/// [`Block`] declarations are generic in [`BlockDomain`]. By convention, the
/// generic domain parameter should be named `D`, and its default value should
/// be [`Sl`]. The declaration can be instantiated with either [`Gl`] or [`Sl`]:
/// 1. `Block<Gl>` is block data that the host provides with the graphics
///    library.
/// 2. `Block<Sl>` gives access to block input data in the shading language.
///
/// User-defined types can implement this trait with a [derive
/// macro](`posh_derive::Block`).
///
/// # Example
///
/// ```
/// use posh::{sl, Block, BlockDomain, Sl};
///
/// #[derive(Clone, Copy, Block)]
/// struct WeirdColor<D: BlockDomain = Sl> {
///     rainbow: D::U32,
///     hell: D::Vec2<f32>,
/// }
///
/// #[derive(Clone, Copy, Block)]
/// struct MyVertex<D: BlockDomain = Sl> {
///     position: D::Vec3<f32>,
///     normal: D::Vec3<f32>,
///     color: WeirdColor<D>,
/// }
///
/// // A function in the shading language that does something with `MyVertex`.
/// fn my_extrude(vertex: MyVertex, offset: sl::F32) -> sl::Vec3<f32> {
///     vertex.position + vertex.normal * offset
/// }
/// ```
///
/// # Safety
///
/// TODO
pub unsafe trait Block<D: BlockDomain>: ToValue {
    /// The representation of [`Self`] in the graphics library domain [`Gl`].
    ///
    /// This is the type through which the host provides block data.
    type InGl: Block<Gl> + AsStd140 + ToValue<Output = Self::InSl>;

    /// The representation of [`Self`] in the shading language domain [`Sl`].
    ///
    /// This is the type through which shaders access block data.
    type InSl: Block<Sl> + Value + ToValue<Output = Self::InSl>;

    #[doc(hidden)]
    fn uniform_input(_path: &str) -> Self {
        unimplemented!()
    }

    #[doc(hidden)]
    fn vertex_input(_path: &str) -> Self {
        unimplemented!()
    }

    #[doc(hidden)]
    fn vertex_attribute_defs(path: &str) -> Vec<VertexAttributeDef> {
        <Self::InSl as Block<Sl>>::vertex_attribute_defs(path)
    }
}

// VertexInterface

/// Provides types for [`VertexInterface`] declarations.
///
/// See [`VertexInterface`] for more details.
#[sealed]
pub trait VertexDomain: Copy {
    /// A vertex field.
    type Vertex<V: Block<Sl>>: VertexInterfaceField<Self>;
}

/// Types that are allowed to occur in a [`VertexInterface`].
#[sealed]
#[doc(hidden)]
pub trait VertexInterfaceField<D: VertexDomain>: Sized {
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

/// A vertex shader input interface.
///
/// Vertex interfaces define how vertex data is passed from the graphics library
/// to the shading language.
///
/// [`VertexInterface`] declarations are generic in [`VertexDomain`]. By
/// convention, the generic domain parameter should be named `D`, and its
/// default value should be [`Sl`]. The declaration can be instantiated with
/// either [`Gl`] or [`Sl`]:
/// 1. `VertexInterface<Gl>` are vertex buffer bindings provided with the
///    graphics library. Each field corresponds to a vertex buffer.
/// 2. `VertexInterface<Sl>` gives access to input vertex data in the shading
///    language. Each field corresponds to the value of the current vertex.
///
/// User-defined types can implement this trait with a [derive
/// macro](`posh_derive::VertexInterface`). Note, however, that types
/// implementing `Block<Sl>` also implement `VertexInterface<Sl>`, so block data
/// can be passed to shaders without declaring a custom `VertexInterface`.
///
/// # Example
///
/// This example declares a vertex interface where `position` is specified in
/// one vertex buffer, and `normal` and `color` are specified in another vertex
/// buffer.
///
/// ```
/// use posh::{
///     sl::{self, VaryingOutput},
///     Block, BlockDomain, VertexDomain, VertexInterface, Sl,
/// };
///
/// #[derive(Clone, Copy, Block)]
/// struct Material<D: BlockDomain = Sl> {
///     normal: D::Vec3<f32>,
///     color: D::Vec4<f32>,
/// }
///
/// #[derive(VertexInterface)]
/// struct MyVertexIface<D: VertexDomain = Sl> {
///     position: D::Vertex<sl::Vec3<f32>>,
///     material: D::Vertex<Material>,
/// }
///
/// // A vertex shader that uses `MyVertexIface`.
/// fn my_vertex_shader(
///     resources: (),
///     vertex: MyVertexIface,
/// ) -> VaryingOutput<sl::Vec4<f32>> {
///     VaryingOutput {
///         position: (vertex.position + vertex.material.normal * 1.0).to_vec4(),
///         varying: vertex.material.color,
///     }
/// }
/// ```
///
/// # Safety
///
/// TODO
pub unsafe trait VertexInterface<D: VertexDomain>: Sized {
    /// The representation of [`Self`] in the graphics library domain [`Gl`].
    ///
    /// Provides vertex buffers for creating a [`crate::gl::VertexArray`](vertex
    /// array) that matches this vertex interface.
    type InGl: VertexInterface<Gl>;

    /// The representation of [`Self`] in the shading language domain [`Sl`].
    ///
    /// The input that user-defined vertex shaders using this vertex interface
    /// receive.
    type InSl: VertexInterface<Sl>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexInterfaceVisitor<'a, D>);

    #[doc(hidden)]
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

pub trait VertexInterfaceVisitor<'a, D: VertexDomain> {
    fn accept<V: Block<Sl>>(
        &mut self,
        path: &str,
        input_rate: VertexInputRate,
        vertex: &'a D::Vertex<V>,
    );
}

// ResourceInterface

/// Provides types for [`ResourceInterface`] declarations.
#[sealed]
pub trait ResourceDomain: Copy {
    /// A two-dimensional sampler field.
    type Sampler2d<T: Numeric>: ResourceInterface<Self>;

    /// A uniform field.
    type Uniform<U: Block<Sl, InSl = U>>: ResourceInterface<Self>;

    /// A resource interface field.
    type Compose<R: ResourceInterface<Sl>>: ResourceInterface<Self>;
}

/// A shader resource input interface.
///
/// Resources give shaders access to uniforms or samplers to shaders that are
/// bound by the host. Resources can be used in both vertex shaders and fragment
/// shaders. In order to link a vertex shader with a fragment shader, they must
/// use the same resource type. See
/// [`create_program`](crate::gl::Context::create_program) for details.
///
/// User-defined types can implement this trait with a [derive
/// macro](`posh_derive::ResourceInterface`).
///
/// # Safety
///
/// TODO
pub unsafe trait ResourceInterface<D: ResourceDomain>: Sized {
    /// The representation of [`Self`] in the graphics library domain [`Gl`].
    ///
    /// Provides resource bindings such as [uniform
    /// buffers](crate::gl::UniformBuffer) or [samplers](crate::gl::Sampler2d).
    /// This is specified on the host through [draw
    /// calls](crate::gl::Program::draw) with programs using this resource
    /// interface.
    type InGl: ResourceInterface<Gl>;

    /// The representation of [`Self`] in the shading language domain [`Sl`].
    ///
    /// The input that user-defined shaders using this resource interface
    /// receive.
    type InSl: ResourceInterface<Sl>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl ResourceInterfaceVisitor<'a, D>);

    #[doc(hidden)]
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

unsafe impl<D: ResourceDomain> ResourceInterface<D> for () {
    type InGl = ();
    type InSl = ();

    fn visit<'a>(&self, _: &str, _: &mut impl ResourceInterfaceVisitor<'a, D>) {}

    fn shader_input(_: &str) -> Self {}
}

#[doc(hidden)]
pub trait ResourceInterfaceVisitor<'a, D: ResourceDomain> {
    fn accept_sampler2d<T: Numeric>(&mut self, path: &str, sampler: &'a D::Sampler2d<T>);

    fn accept_uniform<U: Block<Sl, InSl = U>>(&mut self, path: &str, uniform: &'a D::Uniform<U>);
}

// FragmentInterface

/// Provides types for [`FragmentInterface`] declarations.
#[sealed]
pub trait FragmentDomain: Copy {
    type Attachment: FragmentInterface<Self>;
}

/// A fragment shader output interface.
///
/// User-defined types can implement this trait with a [derive
/// macro](`posh_derive::FragmentInterface`).
///
/// # Safety
///
/// TODO
pub unsafe trait FragmentInterface<D: FragmentDomain> {
    /// The representation of [`Self`] in the graphics library domain [`Gl`].
    ///
    /// Provides framebuffer attachments on the host.
    type InGl: FragmentInterface<Gl>;

    /// The representation of [`Self`] in the shading language domain [`Sl`].
    ///
    /// The output of user-defined fragment shaders using this fragment
    /// interface. Contains expressions describing the shader's output.
    type InSl: FragmentInterface<Sl>;

    #[doc(hidden)]
    fn visit(&self, path: &str, visitor: &mut impl FragmentInterfaceVisitor<D>);
}

#[doc(hidden)]
pub trait FragmentInterfaceVisitor<D: FragmentDomain> {
    fn accept(&mut self, path: &str, attachment: &D::Attachment);
}
