use sealed::sealed;

use crate::{gl, sl, Gl, Sl};

/// A view of fragment data attributes.
///
/// See [`FsBindings`] for more details.
#[sealed]
pub trait FsBindingsDom: Copy {
    type Output<S: sl::ColorSample>: FsBindings<Self>;
}

#[sealed]
impl FsBindingsDom for Gl {
    type Output<S: sl::ColorSample> = gl::ColorAttachment<S>;
}

#[sealed]
impl FsBindingsDom for Sl {
    type Output<S: sl::ColorSample> = S;
}

/// FsBindings shader output data.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::FsBindings`).
///
/// # Safety
///
/// TODO
pub unsafe trait FsBindings<D: FsBindingsDom>: Clone {
    /// The physical view of `Self`.
    ///
    /// This is the type through which framebuffer attachments are provided on
    /// the host.
    type Gl: FsBindings<Gl>;

    /// The logical view of `Self`.
    ///
    /// This is the type through which fragment shaders output fragment data.
    type Sl: FsBindings<Sl> + sl::Value + sl::Varying + sl::ToSl<Output = Self::Sl>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl FragmentVisitor<'a, D>);
}

unsafe impl<D: FsBindingsDom> FsBindings<D> for () {
    type Sl = ();
    type Gl = ();

    fn visit<'a>(&'a self, _: &str, _: &mut impl FragmentVisitor<'a, D>) {}
}

unsafe impl<S: sl::ColorSample> FsBindings<Gl> for gl::ColorAttachment<S> {
    type Gl = gl::ColorAttachment<S>;
    type Sl = S;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl FragmentVisitor<'a, Gl>) {
        visitor.accept(path, self);
    }
}

unsafe impl<S: sl::ColorSample> FsBindings<Sl> for S {
    type Gl = gl::ColorAttachment<S>;
    type Sl = S;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl FragmentVisitor<'a, Sl>) {
        visitor.accept(path, self);
    }
}

#[doc(hidden)]
pub trait FragmentVisitor<'a, D: FsBindingsDom> {
    fn accept<S: sl::ColorSample>(&mut self, path: &str, attachment: &'a D::Output<S>);
}
