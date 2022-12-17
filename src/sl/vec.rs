use std::rc::Rc;

use crate::{
    dag::{BaseTy, Expr, StructTy, Ty},
    Numeric, Primitive,
};

use super::{
    primitives::{field, simplify_struct_literal},
    Object, Scalar, Value,
};

/// A two-dimensional vector in [`Posh`](crate::Posh).
#[derive(Debug, Copy, Clone)]
pub struct Vec2<T> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
}

/// A four-dimensional vector in [`Posh`](crate::Posh).
#[derive(Debug, Copy, Clone)]
pub struct Vec3<T> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
    pub z: Scalar<T>,
}

/// A four-dimensional vector in [`Posh`](crate::Posh).
#[derive(Debug, Copy, Clone)]
pub struct Vec4<T> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
    pub z: Scalar<T>,
    pub w: Scalar<T>,
}

/// Implement `Object` and `Value` for `$ty<T>`.
macro_rules! impl_value {
    ($ty:ident, $name:literal, $($member:ident),+) => {
        impl<T: Primitive> Object for $ty<T> {
            const TY: Ty = Ty::Base(BaseTy::Struct(&Self::STRUCT_TY));

            fn expr(&self) -> Rc<Expr> {
                simplify_struct_literal(
                    &Self::STRUCT_TY,
                    &[$(self.$member.expr()),+],
                )
            }
        }

        impl<T: Primitive> Value for $ty<T> {
            fn from_expr(expr: Expr) -> Self {
                let base = Rc::new(expr);

                Self {
                    $(
                        $member: field(base.clone(), stringify!($member))
                    ),+
                }
            }
        }

        impl<T: Primitive> $ty<T> {
            const STRUCT_TY: StructTy = StructTy {
                // FIXME: Vec*<T> struct name.
                name: $name,
                fields: &[
                    $((stringify!($member), Ty::Base(BaseTy::Scalar(T::PRIMITIVE_TY)))),+
                ],
                is_built_in: true,
            };
        }
    };
}

impl_value!(Vec2, "vec2", x, y);
impl_value!(Vec3, "vec3", x, y, z);
impl_value!(Vec4, "vec4", x, y, z, w);