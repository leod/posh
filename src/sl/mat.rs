use std::{
    ops::{Add, Div, Mul, Neg, Sub},
    rc::Rc,
};

use super::{
    dag::{BinaryOp, BuiltInType, Expr, Type, UnaryOp},
    primitives::{binary, built_in_1, built_in_2, common_field_base, field, unary, value_arg},
    Bool, Object, ToSl, Value, ValueNonArray, Vec2, Vec3, Vec4, F32,
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

        impl ToSl for $mat {
            type Output = Self;

            fn to_sl(self) -> Self::Output {
                self
            }
        }

        impl $mat {
            pub fn eq(self, right: impl ToSl<Output = Self>) -> Bool {
                <Self as Value>::eq(self, right)
            }

            pub fn ne(self, right: impl ToSl<Output = Self>) -> Bool {
                <Self as Value>::ne(self, right)
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
            Rhs: ToSl<Output = $vec>,
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
    ($mat:ident, $vec_ty:ident, ($($member:ident),+), ($($axis:ident),+)) => {
        impl_value!($mat, $($member),+);

        impl_binary_op_symmetric!($mat, add, Add);
        impl_binary_op_symmetric!($mat, div, Div);
        impl_binary_op_symmetric!($mat, mul, Mul);
        impl_binary_op_symmetric!($mat, sub, Sub);

        impl_binary_op_scalar_rhs!($mat, add, Add);
        impl_binary_op_scalar_rhs!($mat, div, Div);
        impl_binary_op_scalar_rhs!($mat, mul, Mul);
        impl_binary_op_scalar_rhs!($mat, sub, Sub);

        impl_binary_op_vec_rhs!($mat, $vec_ty, mul, Mul);

        impl Neg for $mat {
            type Output = Self;

            fn neg(self) -> Self {
                unary(UnaryOp::Neg, self)
            }
        }

        impl Default for $mat {
            fn default() -> Self {
                Self::IDENTITY
            }
        }

        impl $mat {
            pub const ZERO: Self = Self {
                $(
                    $member: $vec_ty::ZERO
                ),+
            };

            pub const IDENTITY: Self = Self {
                $(
                    $member: $vec_ty::$axis
                ),+
            };

            pub fn diagonal(value: impl ToSl<Output = F32>) -> Self {
                built_in_1(&format!("{}", Self::ty()), value.to_sl())
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

            pub fn cmpmul(self, y: Self) -> Self {
                built_in_2("matrixCompMult", self, y)
            }
        }
    };
}

impl_mat!(Mat2, Vec2, (x_axis, y_axis), (X, Y));
impl_mat!(Mat3, Vec3, (x_axis, y_axis, z_axis), (X, Y, Z));
impl_mat!(Mat4, Vec4, (x_axis, y_axis, z_axis, w_axis), (X, Y, Z, W));

impl ToSl for mint::ColumnMatrix2<f32> {
    type Output = Mat2;

    fn to_sl(self) -> Self::Output {
        Self::Output {
            x_axis: self.x.to_sl(),
            y_axis: self.y.to_sl(),
        }
    }
}

impl ToSl for mint::ColumnMatrix3<f32> {
    type Output = Mat3;

    fn to_sl(self) -> Self::Output {
        Self::Output {
            x_axis: self.x.to_sl(),
            y_axis: self.y.to_sl(),
            z_axis: self.z.to_sl(),
        }
    }
}

impl ToSl for mint::ColumnMatrix4<f32> {
    type Output = Mat4;

    fn to_sl(self) -> Self::Output {
        Self::Output {
            x_axis: self.x.to_sl(),
            y_axis: self.y.to_sl(),
            z_axis: self.y.to_sl(),
            w_axis: self.w.to_sl(),
        }
    }
}

/// Creates a two-by-two floating-point matrix from column vectors.
pub fn mat2(x: impl ToSl<Output = Vec2>, y: impl ToSl<Output = Vec2>) -> Mat2 {
    Mat2 {
        x_axis: x.to_sl(),
        y_axis: y.to_sl(),
    }
}

/// Creates a three-by-three floating-point matrix from column vectors.
pub fn mat3(
    x: impl ToSl<Output = Vec3>,
    y: impl ToSl<Output = Vec3>,
    z: impl ToSl<Output = Vec3>,
) -> Mat3 {
    Mat3 {
        x_axis: x.to_sl(),
        y_axis: y.to_sl(),
        z_axis: z.to_sl(),
    }
}

/// Creates a four-by-four floating-point matrix from column vectors.
pub fn mat4(
    x: impl ToSl<Output = Vec4>,
    y: impl ToSl<Output = Vec4>,
    z: impl ToSl<Output = Vec4>,
    w: impl ToSl<Output = Vec4>,
) -> Mat4 {
    Mat4 {
        x_axis: x.to_sl(),
        y_axis: y.to_sl(),
        z_axis: z.to_sl(),
        w_axis: w.to_sl(),
    }
}
