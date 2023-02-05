use std::{
    ops::{Add, Mul, Sub},
    rc::Rc,
};

use crate::dag::{BaseType, BinaryOp, Expr, Type};

use super::{
    primitives::{binary, built_in_1, common_field_base, field, value_arg},
    Object, ToValue, Value, ValueNonArray, Vec2, Vec3, Vec4, F32,
};

/// A two-by-two matrix in the shading language.
#[derive(Debug, Copy, Clone)]
pub struct Mat2 {
    pub x: Vec2<f32>,
    pub y: Vec2<f32>,
}

/// A three-by-three matrix in the shading language.
#[derive(Debug, Copy, Clone)]
pub struct Mat3 {
    pub x: Vec3<f32>,
    pub y: Vec3<f32>,
    pub z: Vec3<f32>,
}

/// A four-by-four matrix in the shading language.
#[derive(Debug, Copy, Clone)]
pub struct Mat4 {
    pub x: Vec4<f32>,
    pub y: Vec4<f32>,
    pub z: Vec4<f32>,
    pub w: Vec4<f32>,
}

// Implements `Object` and `Value` for `$ty`.
macro_rules! impl_value {
    ($ty:ident, $mint_ty: ident, $($member:ident),+) => {
        impl Object for $ty {
            fn ty() -> Type {
                Type::Base(Self::base_type())
            }

            fn expr(&self) -> Rc<Expr> {
                if let Some(base) = common_field_base(
                    &BaseType::$ty,
                    [
                        $(
                            std::stringify!($member)
                        ),+
                    ]
                    .into_iter(),
                    &[
                        $(
                            self.$member.expr()
                        ),+,
                    ],
                ) {
                    base
                } else {
                    let ty = Self::ty();
                    let name = format!("{}", ty);
                    let args = vec![
                        $(
                            self.$member.expr()
                        ),+,
                    ];

                    let expr = Expr::CallBuiltIn { ty, name, args };

                    Rc::new(expr)
                }
            }

            fn from_arg(path: &str) -> Self {
                value_arg(path)
            }
        }

        impl Value for $ty {
            fn from_expr(expr: Expr) -> Self {
                let base = Rc::new(expr);

                Self {
                    $(
                        $member: field(base.clone(), stringify!($member))
                    ),+
                }
            }
        }

        impl ValueNonArray for $ty {
            fn base_type() -> BaseType {
                BaseType::$ty
            }
        }

        impl ToValue for mint::$mint_ty<f32> {
            type Output = $ty;

            fn to_value(self) -> Self::Output {
                Self::Output {
                    $(
                        $member: self.$member.to_value()
                    ),+
                }
            }
        }

        impl ToValue for $ty {
            type Output = Self;

            fn to_value(self) -> Self::Output {
                self
            }
        }
    };
}

// Implements `$ty <op> $ty`.
macro_rules! impl_binary_op_symmetric {
    ($ty:ident, $fn:ident, $op:ident) => {
        impl $op<$ty> for $ty {
            type Output = Self;

            fn $fn(self, right: Self) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

// Implements `$ty <op> $vec_ty`.
macro_rules! impl_binary_op_vec_rhs {
    ($ty:ident, $vec_ty:ident, $fn:ident, $op:ident) => {
        impl<Rhs> $op<Rhs> for $ty
        where
            Rhs: ToValue<Output = $vec_ty<f32>>,
        {
            type Output = $vec_ty<f32>;

            fn $fn(self, right: Rhs) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

// Implements `$ty <op> f32` and `$ty <op> F32`.
macro_rules! impl_binary_op_scalar_rhs {
    ($ty:ident, $fn:ident, $op:ident) => {
        impl $op<f32> for $ty {
            type Output = Self;

            fn $fn(self, right: f32) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<F32> for $ty {
            type Output = Self;

            fn $fn(self, right: F32) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

// Implements all the things for `$ty`.
macro_rules! impl_mat {
    ($ty:ident, $mint_ty:ident, $vec_ty:ident, $($member:ident),+) => {
        impl_value!($ty, $mint_ty, $($member),+);

        impl_binary_op_symmetric!($ty, add, Add);
        impl_binary_op_symmetric!($ty, mul, Mul);
        impl_binary_op_symmetric!($ty, sub, Sub);

        impl_binary_op_vec_rhs!($ty, $vec_ty, mul, Mul);

        impl_binary_op_scalar_rhs!($ty, add, Add);
        impl_binary_op_scalar_rhs!($ty, mul, Mul);
        impl_binary_op_scalar_rhs!($ty, sub, Sub);

        impl Default for $ty {
            fn default() -> Self {
                Self::identity()
            }
        }

        impl $ty {
            pub fn diagonal(value: impl ToValue<Output = F32>) -> Self {
                built_in_1(&format!("{}", Self::ty()), value.to_value())
            }

            pub fn identity() -> Self {
                Self::diagonal(1.0)
            }

            pub fn transpose(self) -> Self {
                built_in_1("transpose", self)
            }

            pub fn determinant(self) -> F32 {
                built_in_1("determinant", self)
            }

            pub fn inverse(self) -> Self {
                built_in_1("inverse", self)
            }
        }
    };
}

impl_mat!(Mat2, ColumnMatrix2, Vec2, x, y);
impl_mat!(Mat3, ColumnMatrix3, Vec3, x, y, z);
impl_mat!(Mat4, ColumnMatrix4, Vec4, x, y, z, w);

/// Constructs a [`Mat2`] column-by-column.
pub fn mat2(x: impl ToValue<Output = Vec2<f32>>, y: impl ToValue<Output = Vec2<f32>>) -> Mat2 {
    Mat2 {
        x: x.to_value(),
        y: y.to_value(),
    }
}

/// Constructs a [`Mat3`] column-by-column.
pub fn mat3(
    x: impl ToValue<Output = Vec3<f32>>,
    y: impl ToValue<Output = Vec3<f32>>,
    z: impl ToValue<Output = Vec3<f32>>,
) -> Mat3 {
    Mat3 {
        x: x.to_value(),
        y: y.to_value(),
        z: z.to_value(),
    }
}

/// Constructs a [`Mat4`] column-by-column.
pub fn mat4(
    x: impl ToValue<Output = Vec4<f32>>,
    y: impl ToValue<Output = Vec4<f32>>,
    z: impl ToValue<Output = Vec4<f32>>,
    w: impl ToValue<Output = Vec4<f32>>,
) -> Mat4 {
    Mat4 {
        x: x.to_value(),
        y: y.to_value(),
        z: z.to_value(),
        w: w.to_value(),
    }
}
