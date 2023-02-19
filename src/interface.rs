mod logical;
mod physical;

use crevice::std140::AsStd140;
use sealed::sealed;

use crate::sl::{
    self,
    program_def::{VertexAttributeDef, VertexInputRate},
};

/// The graphics library's view of shader input and output data.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Physical;

/// The shading language's view of shader input and output data.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Logical;

// Block

/// A view of block attributes.
///
/// See [`Block`] for more details.
#[sealed]
pub trait BlockView: Copy {
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
/// [`VertexData`] or [`UniformData`]. This allows them to be passed to shaders
/// in draw calls.
///
/// `Block` declarations are generic in [`BlockView`] and can be instantiated as
/// their [`Logical`] view or their [`Physical`] view. The views have the
/// following purpose respectively:
///
/// 1. `Block<Logical>` is a view of block data as seen in shader definitions.
///
/// 2. `Block<Physical>` is a view of block data as passed to host-side draw
///    calls in the form of buffer bindings.
///
/// By convention, the generic view parameter of `Block` declarations is named
/// `V`, with [`Logical`] as the default view.
///
/// User-defined types should implement this trait with the [derive
/// macro](`posh_derive::Block`).
///
/// # Example
///
/// ```
/// use posh::{sl, Block, BlockView, Logical};
///
/// #[derive(Clone, Copy, Block)]
/// struct SomeColor<V: BlockView = Logical> {
///     rainbow: V::U32,
///     thing: V::Vec2,
/// }
///
/// #[derive(Clone, Copy, Block)]
/// struct MyVertex<V: BlockView = Logical> {
///     position: V::Vec3,
///     normal: V::Vec3,
///     color: SomeColor<V>,
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
pub unsafe trait Block<V: BlockView>: sl::ToValue {
    /// The logical view of `Self`.
    ///
    /// This is the type through which shaders access block data.
    type Logical: Block<Logical> + sl::Value + sl::ToValue<Output = Self::Logical>;

    /// The physical view of `Self`.
    ///
    /// This is the type through which the host provides block data in draw
    /// calls.
    type Physical: Block<Physical> + AsStd140 + sl::ToValue<Output = Self::Logical>;

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
        <Self::Logical as Block<Logical>>::vertex_attribute_defs(path)
    }
}

// VertexData

/// A view of vertex data attributes.
///
/// See [`VertexData`] for more details.
#[sealed]
pub trait VertexDataView: Copy {
    /// A vertex block field.
    type Block<B: Block<Logical>>: VertexDataField<Self>;
}

/// Types that are allowed to occur in types that implement [`VertexData`].
#[sealed]
#[doc(hidden)]
pub trait VertexDataField<V: VertexDataView>: Sized {
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

/// Vertex shader input data.
///
/// Defines vertex data that can be passed to vertex shaders in draw calls.
///
/// `VertexData` declarations are generic in [`VertexDataView`] and can be
/// instantiated as their [`Logical`] view or their [`Physical`] view. The views
/// have the following purpose respectively:
///
/// 1. `VertexData<Logical>` is a view of vertex data as seen in shader
///    definitions. Each field corresponds to a part of the current vertex
///    value.
///
/// 2. `VertexData<Physical>` is a view of vertex data in the graphics library.
///    Each field is a vertex buffer binding.
///
/// By convention, the generic view parameter is named `V`, with [`Logical`] as
/// the default view.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::VertexData`). Types that implement `Block<Logical>`
/// automatically implement `VertexData<Logical>` as well, so block data can be
/// passed to shaders without having to declare a custom [`VertexData`] type.
///
/// # Example
///
/// This example declares a custom [`VertexData`] type that provides `position`
/// in one vertex buffer, while `normal` and `color` are specified in a second
/// vertex buffer.
///
/// ```
/// use posh::{
///     sl::{self, VaryingOutput},
///     Block, BlockView, Logical, VertexData, VertexDataView,
/// };
///
/// #[derive(Clone, Copy, Block)]
/// struct Material<V: BlockView = Logical> {
///     normal: V::Vec3,
///     color: V::Vec4,
/// }
///
/// #[derive(VertexData)]
/// struct MyVertexData<V: VertexDataView = Logical> {
///     position: V::Block<sl::Vec3>,
///     material: V::Block<Material>,
/// }
///
/// // A vertex shader that receives `MyVertexData` as vertex input.
/// fn my_vertex_shader(
///     uniforms: (),
///     vertex: MyVertexData,
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
pub unsafe trait VertexData<V: VertexDataView>: Sized {
    /// The logical view of `Self`.
    ///
    /// This is the type through which shaders access vertex data.
    type Logical: VertexData<Logical>;

    /// The physical view of `Self`.
    ///
    /// This is the type through which the host provides vertex buffer bindings
    /// for creating [`crate::gl::VertexArray`](vertex arrays).
    type Physical: VertexData<Physical>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexDataVisitor<'a, V>);

    #[doc(hidden)]
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

pub trait VertexDataVisitor<'a, V: VertexDataView> {
    fn accept<B: Block<Logical>>(
        &mut self,
        path: &str,
        input_rate: VertexInputRate,
        vertex: &'a V::Block<B>,
    );
}

// UniformData

/// A view of uniform data attributes.
///
/// See [`UniformData`] for more details.
#[sealed]
pub trait UniformDataView: Copy {
    /// A uniform block field.
    type Block<U: Block<Logical, Logical = U>>: UniformData<Self>;

    /// A two-dimensional uniform sampler field.
    type Sampler2d: UniformData<Self>;

    /// A nested uniform interface field.
    type Compose<R: UniformData<Logical>>: UniformData<Self>;
}

/// Uniform data.
///
/// Defines uniform data that can be passed to shaders in draw calls.
///
/// `UniformData` declarations are generic in [`UniformDataView`] and can be
/// instantiated as their [`Logical`] view or their [`Physical`] view. The views
/// have the following purpose respectively:
///
/// 1. `UniformData<Logical>` is a view of uniform data as seen in shader
///    definitions.
///
/// 2. `UniformData<Physical>` is a view of uniform data in the graphics library
///    containing buffer and sampler bindings.
///
/// In order to use a vertex shader with a fragment shader, they must use the
/// same uniform data in their signature. See
/// [`create_program`](crate::gl::Context::create_program) for details.
///
/// By convention, the generic view parameter is named `V`, with [`Logical`] as
/// the default view.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::UniformData`).
///
/// # Safety
///
/// TODO
pub unsafe trait UniformData<V: UniformDataView>: Sized {
    /// The logical view of `Self`.
    ///
    /// This is the type through which shaders access uniform data.
    type Logical: UniformData<Logical>;

    /// The physical view of `Self`.
    ///
    /// This is the type through which the host provides uniform bindings such
    /// as [uniform buffer bindings](crate::gl::UniformBuffer) or
    /// [samplers](crate::gl::Sampler2d) in [draw calls](crate::gl::Program::draw).
    type Physical: UniformData<Physical>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformDataVisitor<'a, V>);

    #[doc(hidden)]
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

unsafe impl<V: UniformDataView> UniformData<V> for () {
    type Logical = ();
    type Physical = ();

    fn visit<'a>(&self, _: &str, _: &mut impl UniformDataVisitor<'a, V>) {}

    fn shader_input(_: &str) -> Self {}
}

#[doc(hidden)]
pub trait UniformDataVisitor<'a, V: UniformDataView> {
    fn accept_block<U: Block<Logical, Logical = U>>(&mut self, path: &str, block: &'a V::Block<U>);
    fn accept_sampler2d(&mut self, path: &str, sampler: &'a V::Sampler2d);
}

// FragmentData

/// A view fragment data attributes.
///
/// See [`FragmentData`] for more details.
#[sealed]
pub trait FragmentDataView: Copy {
    type Attachment: FragmentData<Self>;
}

/// Fragment shader output data.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::FragmentData`).
///
/// # Safety
///
/// TODO
pub unsafe trait FragmentData<V: FragmentDataView> {
    /// The logical view of `Self`.
    ///
    /// This is the type through which fragment shaders output fragment data.
    type Logical: FragmentData<Logical>;

    /// The physical view of `Self`.
    ///
    /// This is the type through which framebuffer attachments are provided on
    /// the host.
    type Physical: FragmentData<Physical>;

    #[doc(hidden)]
    fn visit(&self, path: &str, visitor: &mut impl FragmentDataVisitor<V>);
}

#[doc(hidden)]
pub trait FragmentDataVisitor<D: FragmentDataView> {
    fn accept(&mut self, path: &str, attachment: &D::Attachment);
}
