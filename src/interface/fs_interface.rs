use sealed::sealed;

use crate::{gl, sl, Gl, Sl, ToSl};

/// A view of fragment data attributes.
///
/// See [`FsInterface`] for more details.
#[sealed]
pub trait FsDom: Copy {
    type ColorAttachment<S: sl::ColorSample>: FsInterface<Self>;
}

#[sealed]
impl FsDom for Gl {
    type ColorAttachment<S: sl::ColorSample> = gl::ColorAttachment<S>;
}

#[sealed]
impl FsDom for Sl {
    type ColorAttachment<S: sl::ColorSample> = S;
}

/// Fragment shader output data.
///
/// User-defined types should implement this trait with a [derive
/// macro](`posh_derive::FsInterface`).
///
/// # Safety
///
/// TODO
pub unsafe trait FsInterface<D: FsDom>: Clone {
    /// The physical view of `Self`.
    ///
    /// This is the type through which framebuffer attachments are provided on
    /// the host.
    type Gl: FsInterface<Gl>;

    /// The logical view of `Self`.
    ///
    /// This is the type through which fragment shaders output fragment data.
    type Sl: FsInterface<Sl> + sl::Interpolant + ToSl<Output = Self::Sl>;

    #[doc(hidden)]
    fn visit<'a>(&'a self, path: &str, visitor: &mut impl FragmentVisitor<'a, D>);
}

unsafe impl<D: FsDom> FsInterface<D> for () {
    type Sl = ();
    type Gl = ();

    fn visit<'a>(&'a self, _: &str, _: &mut impl FragmentVisitor<'a, D>) {}
}

unsafe impl<S: sl::ColorSample> FsInterface<Gl> for gl::ColorAttachment<S> {
    type Gl = gl::ColorAttachment<S>;
    type Sl = S;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl FragmentVisitor<'a, Gl>) {
        visitor.accept(path, self);
    }
}

unsafe impl<S: sl::ColorSample> FsInterface<Sl> for S {
    type Gl = gl::ColorAttachment<S>;
    type Sl = S;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl FragmentVisitor<'a, Sl>) {
        visitor.accept(path, self);
    }
}

// TODO: Tuple implementations for `FsInterface`.

#[doc(hidden)]
pub trait FragmentVisitor<'a, D: FsDom> {
    fn accept<S: sl::ColorSample>(&mut self, path: &str, attachment: &'a D::ColorAttachment<S>);
}
