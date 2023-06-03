use crate::{Block, Gl};

use super::{dag::Expr, Bool, Value, Vec2, Vec4, F32, U32};

/// Constants that can be passed to a shader at shader build time.
///
/// This trait is used to restrict the types that can be used as constants.
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
    pub clip_position: Vec4,
    pub interpolant: W,
    pub point_size: Option<F32>,
}

/// Per-vertex position and interpolant output computed by a vertex shader.
#[derive(Debug, Copy, Clone)]
pub struct VsOutput<W> {
    pub clip_position: Vec4,
    pub interpolant: W,
}

#[derive(Debug, Clone, Copy)]
pub struct Derivatives(pub(super) ());

/// Per-fragment input given to a fragment shader.
#[derive(Debug, Copy, Clone)]
pub struct FsInput<W> {
    pub interpolant: W,
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
pub struct FsOutput<F> {
    pub fragment: F,
    pub fragment_depth: Option<F32>,
}
