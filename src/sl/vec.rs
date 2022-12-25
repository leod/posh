use std::rc::Rc;

use crate::{
    dag::{BaseType, Expr, StructType, Type},
    Primitive,
};

use super::{
    primitives::{field, simplify_struct_literal},
    Object, Scalar, Struct, ToValue, Value,
};

/// A two-dimensional vector in the shading language domain [`Sl`](crate::Sl).
#[derive(Debug, Copy, Clone)]
pub struct Vec2<T> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
}

/// A four-dimensional vector in the shading language domain [`Sl`](crate::Sl).
#[derive(Debug, Copy, Clone)]
pub struct Vec3<T> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
    pub z: Scalar<T>,
}

/// A four-dimensional vector in the shading language domain [`Sl`](crate::Sl).
#[derive(Debug, Copy, Clone)]
pub struct Vec4<T> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
    pub z: Scalar<T>,
    pub w: Scalar<T>,
}

// Implements `Object` and `Value` for `$ty<T>`.
macro_rules! impl_value {
    ($ty:ident, $mint_ty: ident, $name:literal, $($member:ident),+) => {
        impl<T: Primitive> Struct for $ty<T> {
            const STRUCT_TYPE: StructType = StructType {
                // FIXME: Vec*<T> struct name.
                name: $name,
                fields: &[
                    $((stringify!($member), Type::Base(BaseType::Scalar(T::PRIMITIVE_TYPE)))),+
                ],
                is_built_in: true,
            };
        }

        impl<T: Primitive> Object for $ty<T> {
            const TYPE: Type = Type::Base(BaseType::Struct(&Self::STRUCT_TYPE));

            fn expr(&self) -> Rc<Expr> {
                simplify_struct_literal(
                    &Self::STRUCT_TYPE,
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

        impl<T: Primitive> ToValue for mint::$mint_ty<T> {
            type Output = $ty<T>;

            fn to_value(self) -> Self::Output {
                Self::Output {
                    $(
                        $member: self.$member.to_value()
                    ),+
                }
            }
        }

        impl<T: Primitive> ToValue for $ty<T> {
            type Output = Self;

            fn to_value(self) -> Self::Output {
                self
            }
        }
    };
}

impl_value!(Vec2, Vector2, "vec2", x, y);
impl_value!(Vec3, Vector3, "vec3", x, y, z);
impl_value!(Vec4, Vector4, "vec4", x, y, z, w);
