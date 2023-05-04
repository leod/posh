use sealed::sealed;

use crate::{gl, internal::join_ident_path, Gl, Sl};

use super::{Block, BlockDom};

/// A view of vertex data attributes.
///
/// See [`VsBindings`] for more details.
#[sealed]
pub trait VsBindingsDom: BlockDom {
    /// A vertex block field.
    type Block<B: Block<Sl>>: VertexField<Self>;
}

#[sealed]
impl VsBindingsDom for Gl {
    type Block<B: Block<Sl>> = gl::VertexBufferBinding<B>;
}

#[sealed]
impl VsBindingsDom for Sl {
    type Block<B: Block<Sl>> = B;
}

/// VsBindings shader input data.
///
/// Defines vertex data that can be passed to vertex shaders in draw calls.
///
/// `VsBindings` declarations are generic in [`VsBindingsDom`] and can be instantiated
/// as their [`Sl`] view or their [`Gl`] view. The views have the following
/// purpose respectively:
///
/// 1. `VsBindings<Sl>` is a view of vertex data as seen in shader definitions. Each
///    field corresponds to a part of the current vertex value.
///
/// 2. `VsBindings<Gl>` is a view of vertex data in the graphics library. Each field
///    is a vertex buffer binding.
///
/// By convention, the generic view parameter is named `D`, with [`Sl`] as the
/// default view.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::VsBindings`). Types that implement `Block<Sl>`
/// automatically implement `VsBindings<Sl>` as well, so block data can be passed to
/// shaders without having to declare a custom [`VsBindings`] type.
///
/// # Example
///
/// This example declares a custom [`VsBindings`] type that provides `position` in
/// one vertex buffer, while `normal` and `color` are specified in a second
/// vertex buffer.
///
/// ```
/// use posh::{gl, sl, Block, BlockDom, Sl, VsBindings, VsBindingsDom};
///
/// #[derive(Clone, Copy, Block)]
/// #[repr(C)]
/// struct Material<D: BlockDom = Sl> {
///     normal: D::Vec3,
///     color: D::Vec4,
/// }
///
/// #[derive(Clone, Copy, VsBindings)]
/// struct MyVertex<D: VsBindingsDom = Sl> {
///     position: D::Block<sl::Vec3>,
///     material: D::Block<Material>,
/// }
///
/// // A vertex shader that receives `MyVertex` as vertex input.
/// fn my_vertex_shader(
///     uniforms: (),
///     vertex: MyVertex,
/// ) -> sl::VertexOutput<sl::Vec4> {
///     sl::VertexOutput {
///         position: (vertex.position + vertex.material.normal * 1.3).extend(1.0),
///         varying: vertex.material.color,
///     }
/// }
/// ```
///
/// # Safety
///
/// TODO
pub unsafe trait VsBindings<D: VsBindingsDom>: Sized {
    /// The physical view of `Self`.
    ///
    /// This is the type through which the host provides vertex buffer bindings
    /// in draw calls.
    type Gl: VsBindings<Gl>;

    /// The logical view of `Self`.
    ///
    /// This is the type through which shaders access vertex data.
    type Sl: VsBindings<Sl>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexVisitor<'a, D>);

    #[doc(hidden)]
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

unsafe impl<B: Block<Sl>> VsBindings<Gl> for gl::VertexBufferBinding<B> {
    type Gl = gl::VertexBufferBinding<B>;
    type Sl = B::Sl;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexVisitor<'a, Gl>) {
        visitor.accept(path, self)
    }
}

unsafe impl<B: Block<Sl>> VsBindings<Sl> for B {
    type Gl = gl::VertexBufferBinding<B>;
    type Sl = B::Sl;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexVisitor<'a, Sl>) {
        visitor.accept(path, self);
    }

    fn shader_input(path: &str) -> Self {
        B::vertex_input(path)
    }
}

unsafe impl<D: VsBindingsDom> VsBindings<D> for () {
    type Gl = ();
    type Sl = ();

    fn visit<'a>(&'a self, _: &str, _: &mut impl VertexVisitor<'a, D>) {}

    fn shader_input(_: &str) {}
}

unsafe impl<U, V> VsBindings<Gl> for (U, V)
where
    U: VsBindings<Gl>,
    V: VsBindings<Gl>,
{
    type Gl = (U, V);
    type Sl = (U::Sl, V::Sl);

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexVisitor<'a, Gl>) {
        self.0.visit(&join_ident_path(path, "a"), visitor);
        self.1.visit(&join_ident_path(path, "b"), visitor);
    }
}

unsafe impl<U, V> VsBindings<Sl> for (U, V)
where
    U: VsBindings<Sl>,
    V: VsBindings<Sl>,
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

/// Types that are allowed to occur in types that implement [`VsBindings`].
#[sealed]
#[doc(hidden)]
pub trait VertexField<D: VsBindingsDom>: Sized {
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

pub trait VertexVisitor<'a, D: VsBindingsDom> {
    fn accept<B: Block<Sl>>(&mut self, path: &str, vertex: &'a D::Block<B>);
}
