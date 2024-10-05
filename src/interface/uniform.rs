use std::array;

use sealed::sealed;

use crate::{gl, internal::join_ident_path, sl, Gl, Sl};

use super::Block;

/// A view of uniform data attributes.
///
/// See [`Uniform`] for more details.
#[sealed]
pub trait UniformDom: Copy {
    /// A block field.
    type Block<B: Block<Sl, Sl = B>>: Uniform<Self>;

    /// A two-dimensional color sampler field.
    type ColorSampler2d<S: sl::ColorSample>: Uniform<Self>;

    /// A two-dimensional comparison sampler field.
    type ComparisonSampler2d: Uniform<Self>;

    /// A nested uniform interface field.
    type Uniform<U: Uniform<Sl>>: Uniform<Self>;

    /// An array field.
    type Array<U: Uniform<Sl>, const N: usize>: Uniform<Self>;
}

#[sealed]
impl UniformDom for Gl {
    type Block<B: Block<Sl, Sl = B>> = gl::UniformBufferBinding<B>;
    type ColorSampler2d<S: sl::ColorSample> = gl::ColorSampler2d<S>;
    type ComparisonSampler2d = gl::ComparisonSampler2d;
    type Uniform<R: Uniform<Sl>> = R::Gl;
    type Array<U: Uniform<Sl>, const N: usize> = [U::Gl; N];
}

#[sealed]
impl UniformDom for Sl {
    type Block<B: Block<Sl, Sl = B>> = B;
    type ColorSampler2d<S: sl::ColorSample> = sl::ColorSampler2d<S>;
    type ComparisonSampler2d = sl::ComparisonSampler2d;
    type Uniform<R: Uniform<Sl>> = R;
    type Array<U: Uniform<Sl>, const N: usize> = [U; N];
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
/// By convention, the generic view parameter is named `D`.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::Uniform`).
///
/// # Safety
///
/// TODO
pub unsafe trait Uniform<D: UniformDom>: Sized {
    /// The physical view of `Self`.
    ///
    /// This is the type through which the host provides uniform bindings such
    /// as [uniform buffer bindings](crate::gl::UniformBuffer) or
    /// [samplers](crate::gl::ColorSampler2d) in [draw calls](crate::gl::Program::draw).
    type Gl: Uniform<Gl>;

    /// The logical view of `Self`.
    ///
    /// This is the type through which shaders access uniform data.
    type Sl: Uniform<Sl>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, D>);

    #[doc(hidden)]
    fn shader_input(_path: &str) -> Self {
        unimplemented!()
    }
}

unsafe impl<D: UniformDom> Uniform<D> for () {
    type Gl = ();
    type Sl = ();

    fn visit<'a>(&self, _: &str, _: &mut impl UniformVisitor<'a, D>) {}

    fn shader_input(_: &str) -> Self {}
}

unsafe impl<B: Block<Sl, Sl = B>> Uniform<Gl> for gl::UniformBufferBinding<B> {
    type Gl = gl::UniformBufferBinding<B>;
    type Sl = B;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, Gl>) {
        visitor.accept_block::<B::Sl>(path, self);
    }
}

unsafe impl<B: Block<Sl, Sl = B>> Uniform<Sl> for B {
    type Gl = gl::UniformBufferBinding<B>;
    type Sl = B;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, Sl>) {
        visitor.accept_block(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <B as Block<Sl>>::uniform_input(path)
    }
}

unsafe impl<S: sl::ColorSample> Uniform<Gl> for gl::ColorSampler2d<S> {
    type Gl = gl::ColorSampler2d<S>;
    type Sl = sl::ColorSampler2d<S>;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, Gl>) {
        visitor.accept_color_sampler_2d(path, self);
    }
}

unsafe impl<S: sl::ColorSample> Uniform<Sl> for sl::ColorSampler2d<S> {
    type Gl = gl::ColorSampler2d<S>;
    type Sl = Self;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, Sl>) {
        visitor.accept_color_sampler_2d(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <Self as sl::Object>::from_arg(path)
    }
}

unsafe impl Uniform<Gl> for gl::ComparisonSampler2d {
    type Gl = gl::ComparisonSampler2d;
    type Sl = sl::ComparisonSampler2d;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, Gl>) {
        visitor.accept_comparison_sampler_2d(path, self);
    }
}

unsafe impl Uniform<Sl> for sl::ComparisonSampler2d {
    type Gl = gl::ComparisonSampler2d;
    type Sl = sl::ComparisonSampler2d;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, Sl>) {
        visitor.accept_comparison_sampler_2d(path, self)
    }

    fn shader_input(path: &str) -> Self {
        <Self as sl::Object>::from_arg(path)
    }
}

unsafe impl<U, V, D> Uniform<D> for (U, V)
where
    U: Uniform<D>,
    V: Uniform<D>,
    D: UniformDom,
{
    type Gl = (U::Gl, V::Gl);
    type Sl = (U::Sl, V::Sl);

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, D>) {
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

// TODO: Allowing arrays of `Uniform`s might NOT be a good idea. As far as I
// understand, it might preclude us from allowing arrays of `Block`, due to
// conflicting implementations.
unsafe impl<U, D, const N: usize> Uniform<D> for [U; N]
where
    U: Uniform<D>,
    D: UniformDom,
{
    type Gl = [U::Gl; N];
    type Sl = [U::Sl; N];

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl UniformVisitor<'a, D>) {
        for i in 0..N {
            self[i].visit(&join_ident_path(path, &i.to_string()), visitor);
        }
    }

    fn shader_input(path: &str) -> Self {
        array::from_fn(|i| U::shader_input(&join_ident_path(path, &i.to_string())))
    }
}

#[doc(hidden)]
pub trait UniformVisitor<'a, D: UniformDom> {
    fn accept_block<B: Block<Sl, Sl = B>>(&mut self, path: &str, block: &'a D::Block<B>);
    fn accept_color_sampler_2d<S: sl::ColorSample>(
        &mut self,
        path: &str,
        sampler: &'a D::ColorSampler2d<S>,
    );
    fn accept_comparison_sampler_2d(&mut self, path: &str, sampler: &'a D::ComparisonSampler2d);
}

/// Non-empty uniform data.
pub trait UniformNonUnit: Uniform<Sl> {}

impl<B: Block<Sl, Sl = B>> UniformNonUnit for B {}

impl<S: sl::ColorSample> UniformNonUnit for sl::ColorSampler2d<S> {}

impl UniformNonUnit for sl::ComparisonSampler2d {}

impl<U, V> UniformNonUnit for (U, V)
where
    U: Uniform<Sl>,
    V: Uniform<Sl>,
{
}

impl<U, const N: usize> UniformNonUnit for [U; N] where U: Uniform<Sl> {}

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

unsafe impl<U1, U2> UniformUnion<U1, (U1, U2)> for (U1, U2)
where
    U1: UniformNonUnit,
    U2: Uniform<Sl>,
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
    U2: Uniform<Sl>,
{
    fn lhs(self) -> (U1, U2) {
        (self.0, self.1)
    }

    fn rhs(self) -> U1 {
        self.0
    }
}

// TODO: More tuple implementations for `Uniform`.
