use std::{
    ops::{Add, Mul, Sub},
    rc::Rc,
};

use super::{
    dag::{BinaryOp, BuiltInType, Expr, Type},
    primitives::{binary, built_in_1, common_field_base, field, value_arg},
    Object, ToValue, Value, ValueNonArray, Vec2, Vec3, Vec4, F32,
};

/// A two-by-two floating-point matrix.
#[derive(Debug, Copy, Clone)]
pub struct Mat2 {
    pub x_axis: Vec2,
    pub y_axis: Vec2,
}

/// A three-by-three floating-point matrix.
#[derive(Debug, Copy, Clone)]
pub struct Mat3 {
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
}

/// A four-by-four floating-point matrix.
#[derive(Debug, Copy, Clone)]
pub struct Mat4 {
    pub x_axis: Vec4,
    pub y_axis: Vec4,
    pub z_axis: Vec4,
    pub w_axis: Vec4,
}

// Implements `Object` and `Value` for `$mat`.
macro_rules! impl_value {
    ($mat:ident, $($member:ident),+) => {
        impl Object for $mat {
            fn ty() -> Type {
                Type::BuiltIn(BuiltInType::$mat)
            }

            fn expr(&self) -> Rc<Expr> {
                let base = common_field_base(
                    &Self::ty(),
                    [$(std::stringify!($member)),+].into_iter(),
                    &[$(self.$member.expr()),+],
                );

                if let Some(base) = base {
                    base
                } else {
                    let ty = Self::ty();
                    let name = format!("{}", ty);
                    let args = vec![$(self.$member.expr()),+];

                    let expr = Expr::CallBuiltIn { ty, name, args };

                    Rc::new(expr)
                }
            }

            fn from_arg(path: &str) -> Self {
                value_arg(path)
            }
        }

        impl Value for $mat {
            fn from_expr(expr: Expr) -> Self {
                let base = Rc::new(expr);

                Self {
                    $(
                        $member: field(base.clone(), stringify!($member))
                    ),+
                }
            }
        }

        impl ValueNonArray for $mat {}

        impl ToValue for glam::$mat {
            type Output = $mat;

            fn to_value(self) -> Self::Output {
                Self::Output {
                    $(
                        $member: self.$member.to_value()
                    ),+
                }
            }
        }

        impl ToValue for $mat {
            type Output = Self;

            fn to_value(self) -> Self::Output {
                self
            }
        }
    };
}

// Implements `$mat <op> $mat`.
macro_rules! impl_binary_op_symmetric {
    ($mat:ident, $fn:ident, $op:ident) => {
        impl $op<$mat> for $mat {
            type Output = Self;

            fn $fn(self, right: Self) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

// Implements `$mat <op> $vec`.
macro_rules! impl_binary_op_vec_rhs {
    ($mat:ident, $vec:ident, $fn:ident, $op:ident) => {
        impl<Rhs> $op<Rhs> for $mat
        where
            Rhs: ToValue<Output = $vec>,
        {
            type Output = $vec;

            fn $fn(self, right: Rhs) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

// Implements `$mat <op> $scalar`.
macro_rules! impl_binary_op_scalar_rhs {
    ($mat:ident, $fn:ident, $op:ident) => {
        impl $op<f32> for $mat {
            type Output = Self;

            fn $fn(self, right: f32) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<F32> for $mat {
            type Output = Self;

            fn $fn(self, right: F32) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

// Implements all the things for `$mat`.
macro_rules! impl_mat {
    ($mat:ident, $vec_ty:ident, $($member:ident),+) => {
        impl_value!($mat, $($member),+);

        impl_binary_op_symmetric!($mat, add, Add);
        impl_binary_op_symmetric!($mat, mul, Mul);
        impl_binary_op_symmetric!($mat, sub, Sub);

        impl_binary_op_vec_rhs!($mat, $vec_ty, mul, Mul);

        impl_binary_op_scalar_rhs!($mat, add, Add);
        impl_binary_op_scalar_rhs!($mat, mul, Mul);
        impl_binary_op_scalar_rhs!($mat, sub, Sub);

        impl Default for $mat {
            fn default() -> Self {
                Self::identity()
            }
        }

        impl $mat {
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

impl_mat!(Mat2, Vec2, x_axis, y_axis);
impl_mat!(Mat3, Vec3, x_axis, y_axis, z_axis);
impl_mat!(Mat4, Vec4, x_axis, y_axis, z_axis, w_axis);

/// Creates a two-by-two floating-point matrix from column vectors.
pub fn mat2(x: impl ToValue<Output = Vec2>, y: impl ToValue<Output = Vec2>) -> Mat2 {
    Mat2 {
        x_axis: x.to_value(),
        y_axis: y.to_value(),
    }
}

/// Creates a three-by-three floating-point matrix from column vectors.
pub fn mat3(
    x: impl ToValue<Output = Vec3>,
    y: impl ToValue<Output = Vec3>,
    z: impl ToValue<Output = Vec3>,
) -> Mat3 {
    Mat3 {
        x_axis: x.to_value(),
        y_axis: y.to_value(),
        z_axis: z.to_value(),
    }
}

/// Creates a four-by-four floating-point matrix from column vectors.
pub fn mat4(
    x: impl ToValue<Output = Vec4>,
    y: impl ToValue<Output = Vec4>,
    z: impl ToValue<Output = Vec4>,
    w: impl ToValue<Output = Vec4>,
) -> Mat4 {
    Mat4 {
        x_axis: x.to_value(),
        y_axis: y.to_value(),
        z_axis: z.to_value(),
        w_axis: w.to_value(),
    }
}
