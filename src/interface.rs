mod gl_view;
mod sl_view;

use crevice::std140::AsStd140;
use sealed::sealed;

use crate::sl::{
    self,
    program_def::{VertexAttributeDef, VertexInputRate},
    ColorSample,
};

/// The graphics library's view of shader inputs and outputs.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Gl;

/// The shading language's view of shader inputs and outputs.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sl;

// Block

/// A view of block attributes.
///
/// See [`Block`] for more details.
#[sealed]
pub trait BlockDom: Copy {
    /// A floating-point value.
    ///
    /// Has [`f32`] as its physical view and [`sl::F32`] as its logical view.
    type F32: Block<Self> + sl::ToValue<Output = sl::F32>;

    /// A signed integer value.
    ///
    /// Has [`i32`] as its physical view and [`sl::I32`] as its logical view.
    type I32: Block<Self> + sl::ToValue<Output = sl::I32>;

    /// An unsigned integer value.
    ///
    /// Has [`u32`] as its physical view and [`sl::U32`] as its logical view.
    type U32: Block<Self> + sl::ToValue<Output = sl::U32>;

    /// A boolean value.
    ///
    /// Has [`gl::Bool`] as its physical view and [`sl::Bool`] as its logical
    /// view.
    type Bool: Block<Self> + sl::ToValue<Output = sl::Bool>;

    /// A two-dimensional floating-point vector.
    ///
    /// Has [`glam::Vec2`] as its physical view and [`sl::Vec2`] as its logical
    /// view.
    type Vec2: Block<Self> + sl::ToValue<Output = sl::Vec2>;

    /// A three-dimensional floating-point vector.
    ///
    /// Has [`glam::Vec3`] as its physical view and [`sl::Vec3`] as its logical
    /// view.
    type Vec3: Block<Self> + sl::ToValue<Output = sl::Vec3>;

    /// A four-dimensional floating-point vector.
    ///
    /// Has [`glam::Vec4`] as its physical view and [`sl::Vec4`] as its logical
    /// view.
    type Vec4: Block<Self> + sl::ToValue<Output = sl::Vec4>;

    /// A two-by-two floating-point matrix.
    ///
    /// Has [`glam::Mat2`] as its physical view and [`sl::Mat2`] as its logical
    /// view.
    type Mat2: Block<Self> + sl::ToValue<Output = sl::Mat2>;

    /// A three-by-three floating-point matrix.
    ///
    /// Has [`glam::Mat3`] as its physical view and [`sl::Mat3`] as its logical
    /// view.
    type Mat3: Block<Self> + sl::ToValue<Output = sl::Mat3>;

    /// A four-by-four floating-point matrix.
    ///
    /// Has [`glam::Mat4`] as its physical view and [`sl::Mat4`] as its logical
    /// view.
    type Mat4: Block<Self> + sl::ToValue<Output = sl::Mat4>;
}

/// Plain-old vertex or uniform block data.
///
/// Types that implement [`Block`] can be used as fields in types that implement
/// [`Vertex`] or [`Uniform`]. This allows them to be passed to shaders in draw
/// calls.
///
/// `Block` declarations are generic in [`BlockDom`] and can be instantiated as
/// their [`Sl`] view or their [`Gl`] view. The views have the following purpose
/// respectively:
///
/// 1. `Block<Sl>` is a view of block data as seen in shader definitions.
///
/// 2. `Block<Gl>` is a view of block data as passed to host-side draw calls in
///    the form of buffer bindings.
///
/// By convention, the generic view parameter of `Block` declarations is named
/// `D`, with [`Sl`] as the default view.
///
/// User-defined types should implement this trait with the [derive
/// macro](`posh_derive::Block`).
///
/// # Example
///
/// ```
/// use posh::{sl, Block, BlockDom, Sl};
///
/// #[derive(Clone, Copy, Block)]
/// struct SomeColor<D: BlockDom = Sl> {
///     rainbow: D::U32,
///     thing: D::Vec2,
/// }
///
/// #[derive(Clone, Copy, Block)]
/// struct MyVertex<D: BlockDom = Sl> {
///     position: D::Vec3,
///     normal: D::Vec3,
///     color: SomeColor<D>,
/// }
///
/// // A function in the shading language that does something with `MyVertex`.
/// fn my_extrude(vertex: MyVertex, offset: sl::F32) -> sl::Vec3 {
///     vertex.position + vertex.normal * offset
/// }
/// ```
///
/// # Safety
///
/// TODO
pub unsafe trait Block<D: BlockDom>: sl::ToValue {
    /// The logical view of `Self`.
    ///
    /// This is the type through which shaders access block data.
    type Sl: Block<Sl> + sl::Value + sl::ToValue<Output = Self::Sl>;

    /// The physical view of `Self`.
    ///
    /// This is the type through which the host provides block data in draw
    /// calls.
    type Gl: Block<Gl> + AsStd140 + sl::ToValue<Output = Self::Sl>;

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
        <Self::Sl as Block<Sl>>::vertex_attribute_defs(path)
    }
}

// Vertex

/// A view of vertex data attributes.
///
/// See [`Vertex`] for more details.
#[sealed]
pub trait VertexDom: Copy {
    /// A vertex block field.
    type Block<B: Block<Sl>>: VertexField<Self>;
}

/// Types that are allowed to occur in types that implement [`Vertex`].
#[sealed]
#[doc(hidden)]
pub trait VertexField<D: VertexDom>: Sized {
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

/// Vertex shader input data.
///
/// Defines vertex data that can be passed to vertex shaders in draw calls.
///
/// `Vertex` declarations are generic in [`VertexDom`] and can be instantiated
/// as their [`Sl`] view or their [`Gl`] view. The views have the following
/// purpose respectively:
///
/// 1. `Vertex<Sl>` is a view of vertex data as seen in shader definitions. Each
///    field corresponds to a part of the current vertex value.
///
/// 2. `Vertex<Gl>` is a view of vertex data in the graphics library. Each field
///    is a vertex buffer binding.
///
/// By convention, the generic view parameter is named `D`, with [`Sl`] as the
/// default view.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::Vertex`). Types that implement `Block<Sl>`
/// automatically implement `Vertex<Sl>` as well, so block data can be passed to
/// shaders without having to declare a custom [`Vertex`] type.
///
/// # Example
///
/// This example declares a custom [`Vertex`] type that provides `position` in
/// one vertex buffer, while `normal` and `color` are specified in a second
/// vertex buffer.
///
/// ```
/// use posh::{gl, sl, Block, BlockDom, Sl, Vertex, VertexDom};
///
/// #[derive(Clone, Copy, Block)]
/// struct Material<D: BlockDom = Sl> {
///     normal: D::Vec3,
///     color: D::Vec4,
/// }
///
/// #[derive(Vertex)]
/// struct MyVertex<D: VertexDom = Sl> {
///     position: D::Block<sl::Vec3>,
///     material: D::Block<Material>,
/// }
///
/// // A vertex shader that receives `MyVertex` as vertex input.
/// fn my_vertex_shader(
///     uniforms: (),
///     vertex: MyVertex,
/// ) -> sl::VaryingOutput<sl::Vec4> {
///     sl::VaryingOutput {
///         position: (vertex.position + vertex.material.normal * 1.3).extend(1.0),
///         varying: vertex.material.color,
///     }
/// }
/// ```
///
/// # Safety
///
/// TODO
pub unsafe trait Vertex<D: VertexDom>: Sized {
    /// The logical view of `Self`.
    ///
    /// This is the type through which shaders access vertex data.
    type Sl: Vertex<Sl>;

    /// The physical view of `Self`.
    ///
    /// This is the type through which the host provides vertex buffer bindings
    /// in draw calls.
    type Gl: Vertex<Gl>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexVisitor<'a, D>);

    #[doc(hidden)]
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

pub trait VertexVisitor<'a, D: VertexDom> {
    fn accept<B: Block<Sl>>(
        &mut self,
        path: &str,
        input_rate: VertexInputRate,
        vertex: &'a D::Block<B>,
    );
}

// Uniform

/// A view of uniform data attributes.
///
/// See [`Uniform`] for more details.
#[sealed]
pub trait UniformDom: Copy {
    /// A block field.
    type Block<B: Block<Sl, Sl = B>>: Uniform<Self>;

    /// A two-dimensional color sampler field.
    type ColorSampler2d<S: ColorSample>: Uniform<Self>;

    /// A two-dimensional comparison sampler field.
    type ComparisonSampler2d: Uniform<Self>;

    /// A nested uniform interface field.
    type Uniform<U: Uniform<Sl>>: Uniform<Self>;
}

/// Uniform data.
///
/// Defines uniform data that can be passed to shaders in draw calls.
///
/// `Uniform` declarations are generic in [`UniformDom`] and can be instantiated
/// as their [`Sl`] view or their [`Gl`] view. The views have the following
/// purpose respectively:
///
/// 1. `Uniform<Sl>` is a view of uniform data as seen in shader definitions.
///
/// 2. `Uniform<Gl>` is a view of uniform data in the graphics library
///    containing buffer and sampler bindings.
///
/// In order to use a vertex shader with a fragment shader, they must use the
/// same uniform data in their signature. See
/// [`create_program`](crate::gl::Context::create_program) for details.
///
/// By convention, the generic view parameter is named `D`, with [`Sl`] as the
/// default view.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::Uniform`).
///
/// # Safety
///
/// TODO
pub unsafe trait Uniform<D: UniformDom>: Sized {
    /// The logical view of `Self`.
    ///
    /// This is the type through which shaders access uniform data.
    type Sl: Uniform<Sl>;

    /// The physical view of `Self`.
    ///
    /// This is the type through which the host provides uniform bindings such
    /// as [uniform buffer bindings](crate::gl::UniformBuffer) or
    /// [samplers](crate::gl::Sampler2d) in [draw calls](crate::gl::Program::draw).
    type Gl: Uniform<Gl>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, D>);

    #[doc(hidden)]
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

unsafe impl<D: UniformDom> Uniform<D> for () {
    type Sl = ();
    type Gl = ();

    fn visit<'a>(&self, _: &str, _: &mut impl UniformVisitor<'a, D>) {}

    fn shader_input(_: &str) -> Self {}
}

/// A union of two uniform data types.
///
/// # Safety
///
/// TODO
pub unsafe trait UniformUnion<U1, U2>: Uniform<Sl>
where
    U1: Uniform<Sl>,
    U2: Uniform<Sl>,
{
    fn lhs(self) -> U1;
    fn rhs(self) -> U2;
}

/// Non-empty uniform data.
pub trait UniformNonUnit: Uniform<Sl> {}

unsafe impl<U> UniformUnion<U, ()> for U
where
    U: UniformNonUnit,
{
    fn lhs(self) -> U {
        self
    }

    fn rhs(self) {}
}

unsafe impl<U> UniformUnion<(), U> for U
where
    U: UniformNonUnit,
{
    fn lhs(self) {}

    fn rhs(self) -> U {
        self
    }
}

unsafe impl UniformUnion<(), ()> for () {
    fn lhs(self) {}

    fn rhs(self) {}
}

unsafe impl<U> UniformUnion<U, U> for U
where
    U: UniformNonUnit,
{
    fn lhs(self) -> U {
        self
    }

    fn rhs(self) -> U {
        self
    }
}

unsafe impl<U1, U2> UniformUnion<U1, U2> for (U1, U2)
where
    U1: Uniform<Sl>,
    U2: Uniform<Sl>,
{
    fn lhs(self) -> U1 {
        self.0
    }

    fn rhs(self) -> U2 {
        self.1
    }
}

#[doc(hidden)]
pub trait UniformVisitor<'a, D: UniformDom> {
    fn accept_block<B: Block<Sl, Sl = B>>(&mut self, path: &str, block: &'a D::Block<B>);
    fn accept_color_sampler_2d<S: ColorSample>(
        &mut self,
        path: &str,
        sampler: &'a D::ColorSampler2d<S>,
    );
    fn accept_comparison_sampler_2d(&mut self, path: &str, sampler: &'a D::ComparisonSampler2d);
}

// Fragment

/// A view of fragment data attributes.
///
/// See [`Fragment`] for more details.
#[sealed]
pub trait FragmentDom: Copy {
    type Output<S: ColorSample>: Fragment<Self>;
}

/// Fragment shader output data.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::Fragment`).
///
/// # Safety
///
/// TODO
pub unsafe trait Fragment<D: FragmentDom> {
    /// The logical view of `Self`.
    ///
    /// This is the type through which fragment shaders output fragment data.
    type Sl: Fragment<Sl>;

    /// The physical view of `Self`.
    ///
    /// This is the type through which framebuffer attachments are provided on
    /// the host.
    type Gl: Fragment<Gl>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl FragmentVisitor<'a, D>);
}

#[doc(hidden)]
pub trait FragmentVisitor<'a, D: FragmentDom> {
    fn accept<S: ColorSample>(&mut self, path: &str, attachment: &'a D::Output<S>);
}
