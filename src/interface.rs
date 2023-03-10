mod gl_view;
mod sl_view;

use crevice::std140::AsStd140;
use sealed::sealed;

use crate::sl::{
    self,
    program_def::{VertexAttributeDef, VertexInputRate},
    Sample,
};

/// The graphics library's view of shader input and output data.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GlView;

/// The shading language's view of shader input and output data.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SlView;

// Block

/// A view of block attributes.
///
/// See [`Block`] for more details.
#[sealed]
pub trait BlockFields: Copy {
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
/// [`Vertex`] or [`Uniform`]. This allows them to be passed to shaders
/// in draw calls.
///
/// `Block` declarations are generic in [`BlockFields`] and can be instantiated as
/// their [`SlView`] view or their [`GlView`] view. The views have the
/// following purpose respectively:
///
/// 1. `Block<SlView>` is a view of block data as seen in shader definitions.
///
/// 2. `Block<GlView>` is a view of block data as passed to host-side draw
///    calls in the form of buffer bindings.
///
/// By convention, the generic view parameter of `Block` declarations is named
/// `F`, with [`SlView`] as the default view.
///
/// User-defined types should implement this trait with the [derive
/// macro](`posh_derive::Block`).
///
/// # Example
///
/// ```
/// use posh::{sl, Block, BlockFields, SlView};
///
/// #[derive(Clone, Copy, Block)]
/// struct SomeColor<F: BlockFields = SlView> {
///     rainbow: F::U32,
///     thing: F::Vec2,
/// }
///
/// #[derive(Clone, Copy, Block)]
/// struct MyVertex<F: BlockFields = SlView> {
///     position: F::Vec3,
///     normal: F::Vec3,
///     color: SomeColor<F>,
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
pub unsafe trait Block<F: BlockFields>: sl::ToValue {
    /// The logical view of `Self`.
    ///
    /// This is the type through which shaders access block data.
    type SlView: Block<SlView> + sl::Value + sl::ToValue<Output = Self::SlView>;

    /// The physical view of `Self`.
    ///
    /// This is the type through which the host provides block data in draw
    /// calls.
    type GlView: Block<GlView> + AsStd140 + sl::ToValue<Output = Self::SlView>;

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
        <Self::SlView as Block<SlView>>::vertex_attribute_defs(path)
    }
}

// Vertex

/// A view of vertex data attributes.
///
/// See [`Vertex`] for more details.
#[sealed]
pub trait VertexFields: Copy {
    /// A vertex block field.
    type Block<B: Block<SlView>>: VertexField<Self>;
}

/// Types that are allowed to occur in types that implement [`Vertex`].
#[sealed]
#[doc(hidden)]
pub trait VertexField<F: VertexFields>: Sized {
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

/// Vertex shader input data.
///
/// Defines vertex data that can be passed to vertex shaders in draw calls.
///
/// `Vertex` declarations are generic in [`VertexFields`] and can be
/// instantiated as their [`SlView`] view or their [`GlView`] view. The views
/// have the following purpose respectively:
///
/// 1. `Vertex<SlView>` is a view of vertex data as seen in shader
///    definitions. Each field corresponds to a part of the current vertex
///    value.
///
/// 2. `Vertex<GlView>` is a view of vertex data in the graphics library.
///    Each field is a vertex buffer binding.
///
/// By convention, the generic view parameter is named `F`, with [`SlView`] as
/// the default view.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::Vertex`). Types that implement `Block<SlView>`
/// automatically implement `Vertex<SlView>` as well, so block data can be
/// passed to shaders without having to declare a custom [`Vertex`] type.
///
/// # Example
///
/// This example declares a custom [`Vertex`] type that provides `position`
/// in one vertex buffer, while `normal` and `color` are specified in a second
/// vertex buffer.
///
/// ```
/// use posh::{
///     sl::{self, VaryingOutput},
///     Block, BlockFields, SlView, Vertex, VertexFields,
/// };
///
/// #[derive(Clone, Copy, Block)]
/// struct Material<F: BlockFields = SlView> {
///     normal: F::Vec3,
///     color: F::Vec4,
/// }
///
/// #[derive(Vertex)]
/// struct MyVertex<F: VertexFields = SlView> {
///     position: F::Block<sl::Vec3>,
///     material: F::Block<Material>,
/// }
///
/// // A vertex shader that receives `MyVertex` as vertex input.
/// fn my_vertex_shader(
///     uniforms: (),
///     vertex: MyVertex,
/// ) -> VaryingOutput<sl::Vec4> {
///     VaryingOutput {
///         position: (vertex.position + vertex.material.normal * 1.3).extend(1.0),
///         varying: vertex.material.color,
///     }
/// }
/// ```
///
/// # Safety
///
/// TODO
pub unsafe trait Vertex<F: VertexFields>: Sized {
    /// The logical view of `Self`.
    ///
    /// This is the type through which shaders access vertex data.
    type SlView: Vertex<SlView>;

    /// The physical view of `Self`.
    ///
    /// This is the type through which the host provides vertex buffer bindings
    /// for creating [`crate::gl::VertexArray`](vertex arrays).
    type GlView: Vertex<GlView>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexVisitor<'a, F>);

    #[doc(hidden)]
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

pub trait VertexVisitor<'a, F: VertexFields> {
    fn accept<B: Block<SlView>>(
        &mut self,
        path: &str,
        input_rate: VertexInputRate,
        vertex: &'a F::Block<B>,
    );
}

// Uniform

/// A view of uniform data attributes.
///
/// See [`Uniform`] for more details.
#[sealed]
pub trait UniformFields: Copy {
    /// A uniform block field.
    type Block<B: Block<SlView, SlView = B>>: Uniform<Self>;

    /// A two-dimensional uniform sampler field.
    type Sampler2d<S: Sample>: Uniform<Self>;

    /// A nested uniform interface field.
    type Compose<U: Uniform<SlView>>: Uniform<Self>;
}

/// Uniform data.
///
/// Defines uniform data that can be passed to shaders in draw calls.
///
/// `Uniform` declarations are generic in [`UniformFields`] and can be
/// instantiated as their [`SlView`] view or their [`GlView`] view. The views
/// have the following purpose respectively:
///
/// 1. `Uniform<SlView>` is a view of uniform data as seen in shader
///    definitions.
///
/// 2. `Uniform<GlView>` is a view of uniform data in the graphics library
///    containing buffer and sampler bindings.
///
/// In order to use a vertex shader with a fragment shader, they must use the
/// same uniform data in their signature. See
/// [`create_program`](crate::gl::Context::create_program) for details.
///
/// By convention, the generic view parameter is named `F`, with [`SlView`] as
/// the default view.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::Uniform`).
///
/// # Safety
///
/// TODO
pub unsafe trait Uniform<F: UniformFields>: Sized {
    /// The logical view of `Self`.
    ///
    /// This is the type through which shaders access uniform data.
    type SlView: Uniform<SlView>;

    /// The physical view of `Self`.
    ///
    /// This is the type through which the host provides uniform bindings such
    /// as [uniform buffer bindings](crate::gl::UniformBuffer) or
    /// [samplers](crate::gl::Sampler2d) in [draw calls](crate::gl::Program::draw).
    type GlView: Uniform<GlView>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, F>);

    #[doc(hidden)]
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

unsafe impl<F: UniformFields> Uniform<F> for () {
    type SlView = ();
    type GlView = ();

    fn visit<'a>(&self, _: &str, _: &mut impl UniformVisitor<'a, F>) {}

    fn shader_input(_: &str) -> Self {}
}

/// A union of two uniform data types.
///
/// # Safety
///
/// TODO
pub unsafe trait UniformUnion<U1, U2>: Uniform<SlView>
where
    U1: Uniform<SlView>,
    U2: Uniform<SlView>,
{
    fn lhs(self) -> U1;
    fn rhs(self) -> U2;
}

/// Non-empty uniform data.
pub trait UniformNonUnit: Uniform<SlView> {}

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
    U1: Uniform<SlView>,
    U2: Uniform<SlView>,
{
    fn lhs(self) -> U1 {
        self.0
    }

    fn rhs(self) -> U2 {
        self.1
    }
}

#[doc(hidden)]
pub trait UniformVisitor<'a, F: UniformFields> {
    fn accept_block<B: Block<SlView, SlView = B>>(&mut self, path: &str, block: &'a F::Block<B>);
    fn accept_sampler2d<S: Sample>(&mut self, path: &str, sampler: &'a F::Sampler2d<S>);
}

// Fragment

/// A view of fragment data attributes.
///
/// See [`Fragment`] for more details.
#[sealed]
pub trait FragmentFields: Copy {
    type Attachment2d<S: Sample>: Fragment<Self>;
}

/// Fragment shader output data.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::Fragment`).
///
/// # Safety
///
/// TODO
pub unsafe trait Fragment<F: FragmentFields> {
    /// The logical view of `Self`.
    ///
    /// This is the type through which fragment shaders output fragment data.
    type SlView: Fragment<SlView>;

    /// The physical view of `Self`.
    ///
    /// This is the type through which framebuffer attachments are provided on
    /// the host.
    type GlView: Fragment<GlView>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl FragmentVisitor<'a, F>);
}

#[doc(hidden)]
pub trait FragmentVisitor<'a, F: FragmentFields> {
    fn accept<S: Sample>(&mut self, path: &str, attachment: &'a F::Attachment2d<S>);
}
