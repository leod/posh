use sealed::sealed;

use crate::{gl, internal::join_ident_path, Gl, Sl};

use super::{Block, BlockDom};

/// A view of vertex data attributes.
///
/// See [`VsInterface`] for more details.
#[sealed]
pub trait VsInterfaceDom: BlockDom {
    /// A vertex block field.
    type Block<B: Block<Sl>>: VertexField<Self>;
}

#[sealed]
impl VsInterfaceDom for Gl {
    type Block<B: Block<Sl>> = gl::VertexBufferBinding<B>;
}

#[sealed]
impl VsInterfaceDom for Sl {
    type Block<B: Block<Sl>> = B;
}

/// VsInterface shader input data.
///
/// Defines vertex data that can be passed to vertex shaders in draw calls.
///
/// `VsInterface` declarations are generic in [`VsInterfaceDom`] and can be
/// instantiated as their [`Sl`] view or their [`Gl`] view. The views have the
/// following purpose respectively:
///
/// 1. `VsInterface<Sl>` is a view of vertex data as seen in shader definitions.
///    Each field corresponds to a part of the current vertex value.
///
/// 2. `VsInterface<Gl>` is a view of vertex data in the graphics library. Each
///    field is a vertex buffer binding.
///
/// By convention, the generic view parameter is named `D`.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::VsInterface`). Types that implement `Block<Sl>`
/// automatically implement `VsInterface<Sl>` as well, so block data can be
/// passed to shaders without having to declare a custom [`VsInterface`] type.
///
/// # Example
///
/// This example declares a custom [`VsInterface`] type that provides `position`
/// in one vertex buffer, while `normal` and `color` are specified in a second
/// vertex buffer.
///
/// ```
/// use posh::{gl, sl, Block, BlockDom, Sl, VsInterface, VsInterfaceDom};
///
/// #[derive(Clone, Copy, Block)]
/// #[repr(C)]
/// struct Material<D: BlockDom> {
///     normal: D::Vec3,
///     color: D::Vec4,
/// }
///
/// #[derive(Clone, Copy, VsInterface)]
/// struct MyVertex<D: VsInterfaceDom> {
///     position: D::Block<sl::Vec3>,
///     material: D::Block<Material<Sl>>,
/// }
///
/// // A vertex shader that receives `MyVertex` as vertex input.
/// fn my_vertex_shader(
///     uniforms: (),
///     vertex: MyVertex<Sl>,
/// ) -> sl::VsOutput<sl::Vec4> {
///     sl::VsOutput {
///         clip_position: (vertex.position + vertex.material.normal * 1.3).extend(1.0),
///         interpolant: vertex.material.color,
///     }
/// }
/// ```
///
/// # Safety
///
/// TODO
pub unsafe trait VsInterface<D: VsInterfaceDom>: Sized {
    /// The physical view of `Self`.
    ///
    /// This is the type through which the host provides vertex buffer bindings
    /// in draw calls.
    type Gl: VsInterface<Gl>;

    /// The logical view of `Self`.
    ///
    /// This is the type through which shaders access vertex data.
    type Sl: VsInterface<Sl>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexVisitor<'a, D>);

    #[doc(hidden)]
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

unsafe impl<B: Block<Sl>> VsInterface<Gl> for gl::VertexBufferBinding<B> {
    type Gl = gl::VertexBufferBinding<B>;
    type Sl = B::Sl;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexVisitor<'a, Gl>) {
        visitor.accept(path, self)
    }
}

unsafe impl<B: Block<Sl>> VsInterface<Sl> for B {
    type Gl = gl::VertexBufferBinding<B>;
    type Sl = B::Sl;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexVisitor<'a, Sl>) {
        visitor.accept(path, self);
    }

    fn shader_input(path: &str) -> Self {
        B::vertex_input(path)
    }
}

unsafe impl<D: VsInterfaceDom> VsInterface<D> for () {
    type Gl = ();
    type Sl = ();

    fn visit<'a>(&'a self, _: &str, _: &mut impl VertexVisitor<'a, D>) {}

    fn shader_input(_: &str) {}
}

unsafe impl<U, V> VsInterface<Gl> for (U, V)
where
    U: VsInterface<Gl>,
    V: VsInterface<Gl>,
{
    type Gl = (U, V);
    type Sl = (U::Sl, V::Sl);

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexVisitor<'a, Gl>) {
        self.0.visit(&join_ident_path(path, "a"), visitor);
        self.1.visit(&join_ident_path(path, "b"), visitor);
    }
}

unsafe impl<U, V> VsInterface<Sl> for (U, V)
where
    U: VsInterface<Sl>,
    V: VsInterface<Sl>,
{
    type Gl = (U::Gl, V::Gl);
    type Sl = (U, V);

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexVisitor<'a, Sl>) {
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

/// Types that are allowed to occur in types that implement [`VsInterface`].
#[sealed]
#[doc(hidden)]
pub trait VertexField<D: VsInterfaceDom>: Sized {
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

#[sealed]
impl<B: Block<Sl>> VertexField<Gl> for gl::VertexBufferBinding<B> {}

#[sealed]
impl<B: Block<Sl>> VertexField<Sl> for B {
    fn shader_input(path: &str) -> Self {
        B::vertex_input(path)
    }
}

pub trait VertexVisitor<'a, D: VsInterfaceDom> {
    fn accept<B: Block<Sl>>(&mut self, path: &str, vertex: &'a D::Block<B>);
}
