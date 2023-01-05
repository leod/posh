use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use crate::{
    dag::{BaseType, BinaryOp, Expr, StructType, Type},
    Numeric, Primitive,
};

use super::{
    primitives::{binary, field, simplify_struct_literal, value_arg},
    Object, Scalar, Struct, ToValue, Value,
};

/// A two-dimensional vector in the shading language.
#[derive(Debug, Copy, Clone)]
pub struct Vec2<T> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
}

impl<T: Primitive> Vec2<T> {
    pub fn to_vec3(self) -> Vec3<T> {
        Vec3 {
            x: self.x,
            y: self.y,
            z: Default::default(),
        }
    }

    pub fn to_vec4(self) -> Vec4<T> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: Default::default(),
            w: Default::default(),
        }
    }
}

/// A four-dimensional vector in the shading language.
#[derive(Debug, Copy, Clone)]
pub struct Vec3<T> {
    pub x: Scalar<T>,
    pub y: Scalar<T>,
    pub z: Scalar<T>,
}

impl<T: Primitive> Vec3<T> {
    pub fn to_vec4(self) -> Vec4<T> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: Default::default(),
        }
    }
}

/// A four-dimensional vector in the shading language.
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
            //const TYPE: Type = Type::Base(BaseType::$ty(T::PRIMITIVE_TYPE));

            // FIXME: This is a hack so we can use `simplify_struct_literal`,
            // but it breaks other things. Should we perhaps move
            // `simplify_struct_literal` to codegen?
            const TYPE: Type = Type::Base(BaseType::Struct(&Self::STRUCT_TYPE));

            fn expr(&self) -> Rc<Expr> {
                simplify_struct_literal(
                    &Self::STRUCT_TYPE,
                    &[$(self.$member.expr()),+],
                )
            }

            fn from_arg(path: &str) -> Self {
                value_arg(path)
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

// Implements `$ty<T> <op> $ty<T>` for all `T: Numeric`.
macro_rules! impl_binary_op_symmetric {
    ($ty:ident, $fn:ident, $op:ident) => {
        impl<T> $op<$ty<T>> for $ty<T>
        where
            T: Numeric,
        {
            type Output = Self;

            fn $fn(self, right: Self) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

// Implements `$ty<T> <op> Scalar<T>` for all `T: Numeric`.
macro_rules! impl_binary_op_scalar_rhs {
    ($ty:ident, $fn:ident, $op:ident) => {
        impl<T, Rhs> $op<Rhs> for $ty<T>
        where
            T: Numeric,
            Rhs: ToValue<Output = Scalar<T>>,
        {
            type Output = Self;

            fn $fn(self, right: Rhs) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

// Implements all the things for `$ty`.
macro_rules! impl_vec {
    ($ty:ident, $mint_ty: ident, $name:literal, $($member:ident),+) => {
        impl_value!($ty, $mint_ty, $name, $($member),+);

        impl<T: Primitive> Default for $ty<T> {
            fn default() -> Self {
                $ty {
                    $(
                        $member: Default::default()
                    ),*
                }
            }
        }

        impl_binary_op_symmetric!($ty, add, Add);
        impl_binary_op_symmetric!($ty, div, Div);
        impl_binary_op_symmetric!($ty, mul, Mul);
        impl_binary_op_symmetric!($ty, sub, Sub);

        impl_binary_op_scalar_rhs!($ty, add, Add);
        impl_binary_op_scalar_rhs!($ty, div, Div);
        impl_binary_op_scalar_rhs!($ty, mul, Mul);
        impl_binary_op_scalar_rhs!($ty, sub, Sub);
    };
}

impl_vec!(Vec2, Vector2, "vec2", x, y);
impl_vec!(Vec3, Vector3, "vec3", x, y, z);
impl_vec!(Vec4, Vector4, "vec4", x, y, z, w);
