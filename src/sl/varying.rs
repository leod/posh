use std::rc::Rc;

use super::{
    dag::Expr, primitives::value_arg, Mat2, Mat3, Mat4, Object, Value, Vec2, Vec3, Vec4, F32,
};

/// Data passed from a vertex stage to a fragment stage.
///
/// The interface of this trait is a private implementation detail.
pub trait Varying: Value {
    #[doc(hidden)]
    fn shader_outputs(&self, path: &str) -> Vec<(String, Rc<Expr>)>;

    #[doc(hidden)]
    fn shader_input(path: &str) -> Self;
}

impl Varying for () {
    fn shader_outputs(&self, _: &str) -> Vec<(String, Rc<Expr>)> {
        Vec::new()
    }

    fn shader_input(_: &str) -> Self {}
}

macro_rules! impl_varying {
    ($ty:ident) => {
        impl Varying for $ty {
            fn shader_outputs(&self, path: &str) -> Vec<(String, Rc<Expr>)> {
                vec![(path.to_string(), self.expr())]
            }

            fn shader_input(path: &str) -> Self {
                value_arg(path)
            }
        }
    };
}

impl_varying!(F32);
impl_varying!(Vec2);
impl_varying!(Vec3);
impl_varying!(Vec4);
impl_varying!(Mat2);
impl_varying!(Mat3);
impl_varying!(Mat4);

// TODO: Impl Varying for integral types. But make sure to check this somehow:
//
// GLSL ES 3.0: 4.3.6 Output Variables
// > Vertex shader outputs that are, or contain, signed or unsigned integers or
// > integer vectors must be qualified with the interpolation qualifier flat
