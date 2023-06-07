use sealed::sealed;

use crate::{gl, internal::join_ident_path, sl, Gl, Sl};

use super::Block;

/// A view of uniform data attributes.
///
/// See [`UniformInterface`] for more details.
#[sealed]
pub trait UniformInterfaceDom: Copy {
    /// A block field.
    type Block<B: Block<Sl, Sl = B>>: UniformInterface<Self>;

    /// A two-dimensional color sampler field.
    type ColorSampler2d<S: sl::ColorSample>: UniformInterface<Self>;

    /// A two-dimensional comparison sampler field.
    type ComparisonSampler2d: UniformInterface<Self>;

    /// A nested uniform interface field.
    type UniformInterface<U: UniformInterface<Sl>>: UniformInterface<Self>;
}

#[sealed]
impl UniformInterfaceDom for Gl {
    type Block<B: Block<Sl, Sl = B>> = gl::UniformBufferBinding<B>;
    type ColorSampler2d<S: sl::ColorSample> = gl::ColorSampler2d<S>;
    type ComparisonSampler2d = gl::ComparisonSampler2d;
    type UniformInterface<R: UniformInterface<Sl>> = R::Gl;
}

#[sealed]
impl UniformInterfaceDom for Sl {
    type Block<B: Block<Sl, Sl = B>> = B;
    type ColorSampler2d<S: sl::ColorSample> = sl::ColorSampler2d<S>;
    type ComparisonSampler2d = sl::ComparisonSampler2d;
    type UniformInterface<R: UniformInterface<Sl>> = R;
}

/// UniformInterface data.
///
/// Defines uniform data that can be passed to shaders in draw calls.
///
/// `UniformInterface` declarations are generic in [`UniformInterfaceDom`] and can be instantiated
/// as their [`Sl`] view or their [`Gl`] view. The views have the following
/// purpose respectively:
///
/// 1. `UniformInterface<Sl>` is a view of uniform data as seen in shader definitions.
///
/// 2. `UniformInterface<Gl>` is a view of uniform data in the graphics library
///    containing buffer and sampler bindings.
///
/// In order to use a vertex shader with a fragment shader, they must use the
/// same uniform data in their signature. See
/// [`create_program`](crate::gl::Context::create_program) for details.
///
/// By convention, the generic view parameter is named `D`.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::UniformInterface`).
///
/// # Safety
///
/// TODO
pub unsafe trait UniformInterface<D: UniformInterfaceDom>: Sized {
    /// The physical view of `Self`.
    ///
    /// This is the type through which the host provides uniform bindings such
    /// as [uniform buffer bindings](crate::gl::UniformBuffer) or
    /// [samplers](crate::gl::ColorSampler2d) in [draw calls](crate::gl::Program::draw).
    type Gl: UniformInterface<Gl>;

    /// The logical view of `Self`.
    ///
    /// This is the type through which shaders access uniform data.
    type Sl: UniformInterface<Sl>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, D>);

    #[doc(hidden)]
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

unsafe impl<D: UniformInterfaceDom> UniformInterface<D> for () {
    type Gl = ();
    type Sl = ();

    fn visit<'a>(&self, _: &str, _: &mut impl UniformVisitor<'a, D>) {}

    fn shader_input(_: &str) -> Self {}
}

unsafe impl<B: Block<Sl, Sl = B>> UniformInterface<Gl> for gl::UniformBufferBinding<B> {
    type Gl = gl::UniformBufferBinding<B>;
    type Sl = B;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, Gl>) {
        visitor.accept_block::<B::Sl>(path, self);
    }
}

unsafe impl<B: Block<Sl, Sl = B>> UniformInterface<Sl> for B {
    type Gl = gl::UniformBufferBinding<B>;
    type Sl = B;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, Sl>) {
        visitor.accept_block(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <B as Block<Sl>>::uniform_input(path)
    }
}

unsafe impl<S: sl::ColorSample> UniformInterface<Gl> for gl::ColorSampler2d<S> {
    type Gl = gl::ColorSampler2d<S>;
    type Sl = sl::ColorSampler2d<S>;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, Gl>) {
        visitor.accept_color_sampler_2d(path, self);
    }
}

unsafe impl<S: sl::ColorSample> UniformInterface<Sl> for sl::ColorSampler2d<S> {
    type Gl = gl::ColorSampler2d<S>;
    type Sl = Self;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, Sl>) {
        visitor.accept_color_sampler_2d(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <Self as sl::Object>::from_arg(path)
    }
}

unsafe impl UniformInterface<Gl> for gl::ComparisonSampler2d {
    type Gl = gl::ComparisonSampler2d;
    type Sl = sl::ComparisonSampler2d;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, Gl>) {
        visitor.accept_comparison_sampler_2d(path, self);
    }
}

unsafe impl UniformInterface<Sl> for sl::ComparisonSampler2d {
    type Gl = gl::ComparisonSampler2d;
    type Sl = sl::ComparisonSampler2d;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, Sl>) {
        visitor.accept_comparison_sampler_2d(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <Self as sl::Object>::from_arg(path)
    }
}

unsafe impl<U, V> UniformInterface<Gl> for (U, V)
where
    U: UniformInterface<Gl>,
    V: UniformInterface<Gl>,
{
    type Gl = (U, V);
    type Sl = (U::Sl, V::Sl);

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, Gl>) {
        self.0.visit(&join_ident_path(path, "a"), visitor);
        self.1.visit(&join_ident_path(path, "b"), visitor);
    }
}

unsafe impl<U, V> UniformInterface<Sl> for (U, V)
where
    U: UniformInterface<Sl>,
    V: UniformInterface<Sl>,
{
    type Gl = (U::Gl, V::Gl);
    type Sl = (U, V);

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, Sl>) {
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

#[doc(hidden)]
pub trait UniformVisitor<'a, D: UniformInterfaceDom> {
    fn accept_block<B: Block<Sl, Sl = B>>(&mut self, path: &str, block: &'a D::Block<B>);
    fn accept_color_sampler_2d<S: sl::ColorSample>(
        &mut self,
        path: &str,
        sampler: &'a D::ColorSampler2d<S>,
    );
    fn accept_comparison_sampler_2d(&mut self, path: &str, sampler: &'a D::ComparisonSampler2d);
}

/// Non-empty uniform data.
pub trait UniformNonUnit: UniformInterface<Sl> {}

impl<B: Block<Sl, Sl = B>> UniformNonUnit for B {}

impl<S: sl::ColorSample> UniformNonUnit for sl::ColorSampler2d<S> {}

impl UniformNonUnit for sl::ComparisonSampler2d {}

impl<U, V> UniformNonUnit for (U, V)
where
    U: UniformInterface<Sl>,
    V: UniformInterface<Sl>,
{
}

/// A union of two uniform data types.
///
/// # Safety
///
/// TODO
pub unsafe trait UniformUnion<U1, U2>: UniformInterface<Sl>
where
    U1: UniformInterface<Sl>,
    U2: UniformInterface<Sl>,
{
    fn lhs(self) -> U1;
    fn rhs(self) -> U2;
}

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
    U1: UniformInterface<Sl>,
    U2: UniformInterface<Sl>,
{
    fn lhs(self) -> U1 {
        self.0
    }

    fn rhs(self) -> U2 {
        self.1
    }
}

unsafe impl<U1, U2> UniformUnion<U1, (U1, U2)> for (U1, U2)
where
    U1: UniformNonUnit,
    U2: UniformInterface<Sl>,
{
    fn lhs(self) -> U1 {
        self.0
    }

    fn rhs(self) -> (U1, U2) {
        (self.0, self.1)
    }
}

unsafe impl<U1, U2> UniformUnion<(U1, U2), U1> for (U1, U2)
where
    U1: UniformNonUnit,
    U2: UniformInterface<Sl>,
{
    fn lhs(self) -> (U1, U2) {
        (self.0, self.1)
    }

    fn rhs(self) -> U1 {
        self.0
    }
}
