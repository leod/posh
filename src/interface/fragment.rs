use sealed::sealed;

use crate::{gl, sl, Gl, Sl};

/// A view of fragment data attributes.
///
/// See [`Fragment`] for more details.
#[sealed]
pub trait FragmentDom: Copy {
    type Output<S: sl::ColorSample>: Fragment<Self>;
}

#[sealed]
impl FragmentDom for Gl {
    type Output<S: sl::ColorSample> = gl::ColorAttachment<S>;
}

#[sealed]
impl FragmentDom for Sl {
    type Output<S: sl::ColorSample> = S;
}

/// Fragment shader output data.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::Fragment`).
///
/// # Safety
///
/// TODO
pub unsafe trait Fragment<D: FragmentDom>: Clone {
    /// The physical view of `Self`.
    ///
    /// This is the type through which framebuffer attachments are provided on
    /// the host.
    type Gl: Fragment<Gl>;

    /// The logical view of `Self`.
    ///
    /// This is the type through which fragment shaders output fragment data.
    type Sl: Fragment<Sl>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl FragmentVisitor<'a, D>);
}

unsafe impl<D: FragmentDom> Fragment<D> for () {
    type Sl = ();
    type Gl = ();

    fn visit<'a>(&'a self, _: &str, _: &mut impl FragmentVisitor<'a, D>) {}
}

unsafe impl<S: sl::ColorSample> Fragment<Gl> for gl::ColorAttachment<S> {
    type Gl = gl::ColorAttachment<S>;
    type Sl = S;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl FragmentVisitor<'a, Gl>) {
        visitor.accept(path, self);
    }
}

unsafe impl<S: sl::ColorSample> Fragment<Sl> for S {
    type Gl = gl::ColorAttachment<S>;
    type Sl = S;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl FragmentVisitor<'a, Sl>) {
        visitor.accept(path, self);
    }
}

#[doc(hidden)]
pub trait FragmentVisitor<'a, D: FragmentDom> {
    fn accept<S: sl::ColorSample>(&mut self, path: &str, attachment: &'a D::Output<S>);
}
