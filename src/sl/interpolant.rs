use std::rc::Rc;

use crate::internal::join_ident_path;

use super::{
    dag::Expr, primitives::value_arg, program_def::InterpolationQualifier, IVec2, IVec3, IVec4,
    Mat2, Mat3, Mat4, Object, UVec2, UVec3, UVec4, Value, Vec2, Vec3, Vec4, F32, I32, U32,
};

/// Data passed from a vertex shader to a fragment shader.
///
/// The interface of this trait is a private implementation detail.
pub unsafe trait Interpolant: Value {
    #[doc(hidden)]
    fn shader_outputs(&self, path: &str) -> Vec<(String, InterpolationQualifier, Rc<Expr>)>;

    #[doc(hidden)]
    fn shader_input(path: &str) -> Self;
}

macro_rules! base_impl {
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

base_impl!(F32, Smooth);
base_impl!(Vec2, Smooth);
base_impl!(Vec3, Smooth);
base_impl!(Vec4, Smooth);
base_impl!(Mat2, Smooth);
base_impl!(Mat3, Smooth);
base_impl!(Mat4, Smooth);

// GLSL ES 3.0: 4.3.6 Output Variables
// > Vertex shader outputs that are, or contain, signed or unsigned integers or
// > integer vectors must be qualified with the interpolation qualifier flat
base_impl!(I32, Flat);
base_impl!(IVec2, Flat);
base_impl!(IVec3, Flat);
base_impl!(IVec4, Flat);
base_impl!(U32, Flat);
base_impl!(UVec2, Flat);
base_impl!(UVec3, Flat);
base_impl!(UVec4, Flat);

macro_rules! tuple_impl {
    ($($name: ident),*) => {
        unsafe impl<$($name: Interpolant,)*> Interpolant for ($($name,)*) {
            #[allow(unused)]
            fn shader_outputs(
                &self,
                path: &str,
            ) -> Vec<(String, InterpolationQualifier, Rc<Expr>)> {
                // TODO: `shader_outputs` should take an optional interpolation
                // qualifier that we can distribute to tuple elements.
                #[allow(non_snake_case)]
                let ($($name,)*) = self;

                let mut result = Vec::new();

                $(
                    result.extend($name.shader_outputs(&join_ident_path(path, stringify!($name))));
                )*

                result
            }

            #[allow(unused)]
            fn shader_input(path: &str) -> Self {
                (
                    $($name::shader_input(&join_ident_path(path, stringify!($name))),)*
                )
            }
        }
    };
}

smaller_tuples_too!(tuple_impl, T0, T1, T2, T3, T4, T5, T6, T7);
