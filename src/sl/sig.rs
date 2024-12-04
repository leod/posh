use crate::{Block, FsInterface, Gl, Sl, Uniform, VsInterface};

use super::{dag::Expr, Bool, Interpolant, Value, Vec2, Vec4, F32, U32};

/// Constants that can be passed to a shader at shader build time.
///
/// This trait is used to restrict the types that can be used as constants.
///
/// # Safety
///
/// TODO
pub unsafe trait Const {}

unsafe impl Const for () {}
unsafe impl Const for usize {}
unsafe impl Const for isize {}
unsafe impl Const for bool {}
unsafe impl Const for String {}
unsafe impl<B: Block<Gl>> Const for B {}
unsafe impl<T: Const> Const for Vec<T> {}
unsafe impl<T: Const> Const for Option<T> {}
unsafe impl<U: Const, V: Const> Const for (U, V) {}
unsafe impl<T: Const, const N: usize> Const for [T; N] {}

/// Per-vertex input given to a vertex shader.
#[derive(Debug, Copy, Clone)]
pub struct VsInput<V> {
    pub vertex: V,
    pub vertex_id: U32,
    pub instance_id: U32,
    pub(crate) _private: (),
}

/// Per-vertex output computed by a vertex shader.
#[derive(Debug, Copy, Clone)]
pub struct FullVsOutput<W> {
    pub clip_pos: Vec4,
    pub interp: W,
    pub point_size: Option<F32>,
}

/// Per-vertex position and interp output computed by a vertex shader.
#[derive(Debug, Copy, Clone)]
pub struct VsOutput<W> {
    pub clip_pos: Vec4,
    pub interp: W,
}

#[derive(Debug, Clone, Copy)]
pub struct Derivatives(pub(super) ());

/// Per-fragment input given to a fragment shader.
#[derive(Debug, Copy, Clone)]
pub struct FsInput<W> {
    pub interp: W,
    pub fragment_coord: Vec4,
    pub front_facing: Bool,
    pub point_coord: Vec2,
    pub derivatives: Derivatives,
}

impl<W> FsInput<W> {
    pub fn discard<V: Value>(self) -> V {
        let ty = V::ty();

        V::from_expr(Expr::Discard { ty })
    }
}

/// Per-fragment output computed by a fragment shader.
#[derive(Debug, Copy, Clone)]
pub struct FullFsOutput<F> {
    pub fragment: F,
    pub fragment_depth: Option<F32>,
}

/// Types that can be used as vertex input for a vertex shader.
pub trait FromVsInput {
    type V: VsInterface<Sl>;

    fn from_vs_input(input: VsInput<Self::V>) -> Self;
}

impl<V: VsInterface<Sl>> FromVsInput for VsInput<V> {
    type V = V;

    fn from_vs_input(input: Self) -> Self {
        input
    }
}

impl<V: VsInterface<Sl>> FromVsInput for V {
    type V = Self;

    fn from_vs_input(input: VsInput<Self>) -> Self {
        input.vertex
    }
}

/// Types that can be used as vertex output for a vertex shader.
pub trait IntoFullVsOutput {
    type W: Interpolant;

    fn into_full_vs_output(self) -> FullVsOutput<Self::W>;
}

impl<W: Interpolant> IntoFullVsOutput for FullVsOutput<W> {
    type W = W;

    fn into_full_vs_output(self) -> Self {
        self
    }
}

impl<V: Interpolant> IntoFullVsOutput for VsOutput<V> {
    type W = V;

    fn into_full_vs_output(self) -> FullVsOutput<V> {
        FullVsOutput {
            clip_pos: self.clip_pos,
            interp: self.interp,
            point_size: None,
        }
    }
}

impl IntoFullVsOutput for Vec4 {
    type W = ();

    fn into_full_vs_output(self) -> FullVsOutput<()> {
        FullVsOutput {
            clip_pos: self,
            interp: (),
            point_size: None,
        }
    }
}

/// Types that can be used as fragment input for a fragment shader.
pub trait FromFsInput {
    type W: Interpolant;

    fn from_fs_input(input: FsInput<Self::W>) -> Self;
}

impl<W: Interpolant> FromFsInput for FsInput<W> {
    type W = W;

    fn from_fs_input(input: Self) -> Self {
        input
    }
}

impl<W: Interpolant> FromFsInput for W {
    type W = Self;

    fn from_fs_input(input: FsInput<Self>) -> Self {
        input.interp
    }
}

/// Types that can be used as fragment output for a fragment shader.
pub trait IntoFullFsOutput {
    type F: FsInterface<Sl>;

    fn into_full_fs_output(self) -> FullFsOutput<Self::F>;
}

impl<F: FsInterface<Sl>> IntoFullFsOutput for FullFsOutput<F> {
    type F = F;

    fn into_full_fs_output(self) -> Self {
        self
    }
}

impl<F: FsInterface<Sl>> IntoFullFsOutput for F {
    type F = Self;

    fn into_full_fs_output(self) -> FullFsOutput<Self> {
        FullFsOutput {
            fragment: self,
            fragment_depth: None,
        }
    }
}

pub trait VsSig {
    type C: Const;
    type U: Uniform<Sl>;
    type V: VsInterface<Sl>;
    type W: Interpolant;
}

/// Function types that define a vertex shader.
pub trait VsFunc<Sig: VsSig>: 'static {
    fn call(
        self,
        consts: &Sig::C,
        uniforms: Sig::U,
        input: VsInput<Sig::V>,
    ) -> FullVsOutput<Sig::W>;
}

macro_rules! impl_vs_func {
    ($v:ident, $v_in:ty, $w:ident, $w_out:ty) => {
        impl<C, U, $v, $w> VsSig for fn(&C, U, $v_in) -> $w_out
        where
            C: Const,
            U: Uniform<Sl>,
            V: VsInterface<Sl>,
            W: Interpolant,
        {
            type C = C;
            type U = U;
            type V = $v;
            type W = $w;
        }

        impl<C, U, $v, $w, Func> VsFunc<fn(&C, U, $v_in) -> $w_out> for Func
        where
            C: Const,
            U: Uniform<Sl>,
            V: VsInterface<Sl>,
            W: Interpolant,
            Func: Fn(&C, U, $v_in) -> $w_out + 'static,
        {
            fn call(self, consts: &C, uniforms: U, input: VsInput<$v>) -> FullVsOutput<$w> {
                self(consts, uniforms, <$v_in>::from_vs_input(input)).into_full_vs_output()
            }
        }

        impl<U, $v, $w> VsSig for fn(U, $v_in) -> $w_out
        where
            U: Uniform<Sl>,
            V: VsInterface<Sl>,
            W: Interpolant,
        {
            type C = ();
            type U = U;
            type V = $v;
            type W = $w;
        }

        impl<U, $v, $w, Func> VsFunc<fn(U, $v_in) -> $w_out> for Func
        where
            U: Uniform<Sl>,
            V: VsInterface<Sl>,
            W: Interpolant,
            Func: Fn(U, $v_in) -> $w_out + 'static,
        {
            fn call(self, _: &(), uniforms: U, input: VsInput<$v>) -> FullVsOutput<$w> {
                self(uniforms, <$v_in>::from_vs_input(input)).into_full_vs_output()
            }
        }

        impl<$v, $w> VsSig for fn($v_in) -> $w_out
        where
            V: VsInterface<Sl>,
            W: Interpolant,
        {
            type C = ();
            type U = ();
            type V = $v;
            type W = $w;
        }

        impl<$v, $w, Func> VsFunc<fn($v_in) -> $w_out> for Func
        where
            V: VsInterface<Sl>,
            W: Interpolant,
            Func: Fn($v_in) -> $w_out + 'static,
        {
            fn call(self, _: &(), _: (), input: VsInput<$v>) -> FullVsOutput<$w> {
                self(<$v_in>::from_vs_input(input)).into_full_vs_output()
            }
        }
    };
    ($v:ident, $v_in:ty) => {
        impl<C, U, $v> VsSig for fn(&C, U, $v_in) -> Vec4
        where
            C: Const,
            U: Uniform<Sl>,
            V: VsInterface<Sl>,
        {
            type C = C;
            type U = U;
            type V = $v;
            type W = ();
        }

        impl<C, U, $v, Func> VsFunc<fn(&C, U, $v_in) -> Vec4> for Func
        where
            C: Const,
            U: Uniform<Sl>,
            V: VsInterface<Sl>,
            Func: Fn(&C, U, $v_in) -> Vec4 + 'static,
        {
            fn call(self, consts: &C, uniforms: U, input: VsInput<$v>) -> FullVsOutput<()> {
                self(consts, uniforms, <$v_in>::from_vs_input(input)).into_full_vs_output()
            }
        }

        impl<U, $v> VsSig for fn(U, $v_in) -> Vec4
        where
            U: Uniform<Sl>,
            V: VsInterface<Sl>,
        {
            type C = ();
            type U = U;
            type V = $v;
            type W = ();
        }

        impl<U, $v, Func> VsFunc<fn(U, $v_in) -> Vec4> for Func
        where
            U: Uniform<Sl>,
            V: VsInterface<Sl>,
            Func: Fn(U, $v_in) -> Vec4 + 'static,
        {
            fn call(self, _: &(), uniforms: U, input: VsInput<$v>) -> FullVsOutput<()> {
                self(uniforms, <$v_in>::from_vs_input(input)).into_full_vs_output()
            }
        }

        impl<$v> VsSig for fn($v_in) -> Vec4
        where
            V: VsInterface<Sl>,
        {
            type C = ();
            type U = ();
            type V = $v;
            type W = ();
        }

        impl<$v, Func> VsFunc<fn($v_in) -> Vec4> for Func
        where
            V: VsInterface<Sl>,
            Func: Fn($v_in) -> Vec4 + 'static,
        {
            fn call(self, _: &(), _: (), input: VsInput<$v>) -> FullVsOutput<()> {
                self(<$v_in>::from_vs_input(input)).into_full_vs_output()
            }
        }
    };
}

impl_vs_func!(V, V);
impl_vs_func!(V, VsInput<V>);
impl_vs_func!(V, V, W, VsOutput<W>);
impl_vs_func!(V, VsInput<V>, W, VsOutput<W>);
impl_vs_func!(V, V, W, FullVsOutput<W>);
impl_vs_func!(V, VsInput<V>, W, FullVsOutput<W>);

pub trait FsSig {
    type C: Const;
    type U: Uniform<Sl>;
    type W: Interpolant;
    type F: FsInterface<Sl>;
}

/// Function types that define a fragment shader.
pub trait FsFunc<Sig: FsSig>: 'static {
    fn call(
        self,
        consts: &Sig::C,
        uniforms: Sig::U,
        input: FsInput<Sig::W>,
    ) -> FullFsOutput<Sig::F>;
}

macro_rules! impl_fs_func {
    ($w:ident, $w_in:ty, $f:ident, $f_out:ty) => {
        impl<C, U, $w, $f> FsSig for fn(&C, U, $w_in) -> $f_out
        where
            C: Const,
            U: Uniform<Sl>,
            W: Interpolant,
            F: FsInterface<Sl>,
        {
            type C = C;
            type U = U;
            type W = $w;
            type F = $f;
        }

        impl<C, U, $w, $f, Func> FsFunc<fn(&C, U, $w_in) -> $f_out> for Func
        where
            C: Const,
            U: Uniform<Sl>,
            W: Interpolant,
            F: FsInterface<Sl>,
            Func: Fn(&C, U, $w_in) -> $f_out + 'static,
        {
            fn call(self, consts: &C, uniforms: U, input: FsInput<$w>) -> FullFsOutput<$f> {
                self(consts, uniforms, <$w_in>::from_fs_input(input)).into_full_fs_output()
            }
        }

        impl<U, $w, $f> FsSig for fn(U, $w_in) -> $f_out
        where
            U: Uniform<Sl>,
            W: Interpolant,
            F: FsInterface<Sl>,
        {
            type C = ();
            type U = U;
            type W = $w;
            type F = $f;
        }

        impl<U, $w, $f, Func> FsFunc<fn(U, $w_in) -> $f_out> for Func
        where
            U: Uniform<Sl>,
            W: Interpolant,
            F: FsInterface<Sl>,
            Func: Fn(U, $w_in) -> $f_out + 'static,
        {
            fn call(self, _: &(), uniforms: U, input: FsInput<$w>) -> FullFsOutput<$f> {
                self(uniforms, <$w_in>::from_fs_input(input)).into_full_fs_output()
            }
        }

        impl<$w, $f> FsSig for fn($w_in) -> $f_out
        where
            W: Interpolant,
            F: FsInterface<Sl>,
        {
            type C = ();
            type U = ();
            type W = $w;
            type F = $f;
        }

        impl<$w, $f, Func> FsFunc<fn($w_in) -> $f_out> for Func
        where
            W: Interpolant,
            F: FsInterface<Sl>,
            Func: Fn($w_in) -> $f_out + 'static,
        {
            fn call(self, _: &(), _: (), input: FsInput<$w>) -> FullFsOutput<$f> {
                self(<$w_in>::from_fs_input(input)).into_full_fs_output()
            }
        }
    };
}

impl_fs_func!(W, W, F, F);
impl_fs_func!(W, W, F, FullFsOutput<F>);
impl_fs_func!(W, FsInput<W>, F, F);
impl_fs_func!(W, FsInput<W>, F, FullFsOutput<F>);
