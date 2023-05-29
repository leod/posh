use std::rc::Rc;

use super::{
    dag::Expr, primitives::value_arg, program_def::InterpolationQualifier, IVec2, IVec3, IVec4,
    Mat2, Mat3, Mat4, Object, UVec2, UVec3, UVec4, Value, Vec2, Vec3, Vec4, F32, I32, U32,
};

/// Data passed from a vertex stage to a fragment stage.
///
/// The interface of this trait is a private implementation detail.
pub unsafe trait Interpolant: Value {
    #[doc(hidden)]
    fn shader_outputs(&self, path: &str) -> Vec<(String, InterpolationQualifier, Rc<Expr>)>;

    #[doc(hidden)]
    fn shader_input(path: &str) -> Self;
}

unsafe impl Interpolant for () {
    fn shader_outputs(&self, _: &str) -> Vec<(String, InterpolationQualifier, Rc<Expr>)> {
        Vec::new()
    }

    fn shader_input(_: &str) -> Self {}
}

macro_rules! impl_interpolant {
    ($ty:ident, $interp:ident) => {
        unsafe impl Interpolant for $ty {
            fn shader_outputs(
                &self,
                path: &str,
            ) -> Vec<(String, InterpolationQualifier, Rc<Expr>)> {
                vec![(
                    path.to_string(),
                    InterpolationQualifier::$interp,
                    self.expr(),
                )]
            }

            fn shader_input(path: &str) -> Self {
                value_arg(path)
            }
        }
    };
}

impl_interpolant!(F32, Smooth);
impl_interpolant!(Vec2, Smooth);
impl_interpolant!(Vec3, Smooth);
impl_interpolant!(Vec4, Smooth);
impl_interpolant!(Mat2, Smooth);
impl_interpolant!(Mat3, Smooth);
impl_interpolant!(Mat4, Smooth);

// GLSL ES 3.0: 4.3.6 Output Variables
// > Vertex shader outputs that are, or contain, signed or unsigned integers or
// > integer vectors must be qualified with the interpolation qualifier flat
impl_interpolant!(I32, Flat);
impl_interpolant!(IVec2, Flat);
impl_interpolant!(IVec3, Flat);
impl_interpolant!(IVec4, Flat);
impl_interpolant!(U32, Flat);
impl_interpolant!(UVec2, Flat);
impl_interpolant!(UVec3, Flat);
impl_interpolant!(UVec4, Flat);
