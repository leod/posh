use std::ops::{Add, Div, Mul, Sub};

use crate::{
    lang::{BinaryOp, BuiltInTy, Expr, Ty},
    IntoValue, Value,
};

use super::{
    binary, builtin3, builtin4, field, scalar::NumericType, BuiltIn, HasValue, Scalar, ScalarType,
    Trace, Transparent, Type,
};

impl<T: ScalarType> Type for [T; 3] {
    fn ty() -> Ty {
        Ty::BuiltIn(Self::built_in_ty())
    }
}

impl<T: ScalarType> Transparent for [T; 3] {
    fn transparent() {}
}

impl<T: ScalarType> Type for [T; 4] {
    fn ty() -> Ty {
        Ty::BuiltIn(Self::built_in_ty())
    }
}

impl<T: ScalarType> Transparent for [T; 4] {
    fn transparent() {}
}

impl<T: ScalarType> BuiltIn for [T; 3] {
    fn built_in_ty() -> BuiltInTy {
        BuiltInTy::Vec3(T::scalar_ty())
    }
}

impl<T: ScalarType> BuiltIn for [T; 4] {
    fn built_in_ty() -> BuiltInTy {
        BuiltInTy::Vec4(T::scalar_ty())
    }
}

#[must_use]
#[derive(Debug, Copy, Clone)]
pub struct Vec3<T> {
    trace: Trace,
    pub x: Scalar<T>,
    pub y: Scalar<T>,
    pub z: Scalar<T>,
}

impl<T: ScalarType> Value for Vec3<T> {
    type Type = [T; 3];

    fn from_trace(trace: Trace) -> Self {
        assert!(trace.expr().ty() == <Self::Type as Type>::ty());

        Self {
            trace,
            x: field(trace, "x"),
            y: field(trace, "y"),
            z: field(trace, "z"),
        }
    }

    fn expr(&self) -> Expr {
        self.trace.expr()
    }
}

impl<T: ScalarType> HasValue for [T; 3] {
    type Value = Vec3<T>;
}

impl<T: ScalarType> HasValue for [T; 4] {
    type Value = Vec4<T>;
}

impl<T: ScalarType> IntoValue for [T; 3] {
    fn into_value(self) -> Self::Value {
        vec3(self[0], self[1], self[2])
    }
}

impl<T: ScalarType> IntoValue for [T; 4] {
    fn into_value(self) -> Self::Value {
        vec4(self[0], self[1], self[2], self[3])
    }
}

#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct Vec4<T> {
    trace: Trace,
    pub x: Scalar<T>,
    pub y: Scalar<T>,
    pub z: Scalar<T>,
    pub w: Scalar<T>,
}

impl<T: ScalarType> Value for Vec4<T> {
    type Type = [T; 4];

    fn from_trace(trace: Trace) -> Self {
        assert!(trace.expr().ty() == <Self::Type as Type>::ty());

        Self {
            trace,
            x: field(trace, "x"),
            y: field(trace, "y"),
            z: field(trace, "z"),
            w: field(trace, "w"),
        }
    }

    fn expr(&self) -> Expr {
        self.trace.expr()
    }
}

macro_rules! impl_symmetric_binary_op {
    ($ty:ident, $fn:ident, $op:ident) => {
        impl<T> $op<$ty<T>> for $ty<T>
        where
            T: NumericType,
        {
            type Output = Self;

            fn $fn(self, right: Self) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

macro_rules! impl_scalar_binary_op {
    ($ty:ident, $fn:ident, $op:ident) => {
        impl<T, Rhs> $op<Rhs> for $ty<T>
        where
            T: NumericType,
            Rhs: IntoValue<Value = Scalar<T>>,
        {
            type Output = Self;

            fn $fn(self, right: Rhs) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl<T> $op<$ty<T>> for Scalar<T>
        where
            T: NumericType,
        {
            type Output = $ty<T>;

            fn $fn(self, right: $ty<T>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<$ty<Self>> for f32 {
            type Output = $ty<Self>;

            fn $fn(self, right: $ty<Self>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<$ty<Self>> for i32 {
            type Output = $ty<Self>;

            fn $fn(self, right: $ty<Self>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<$ty<Self>> for u32 {
            type Output = $ty<Self>;

            fn $fn(self, right: $ty<Self>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

macro_rules! impl_ops {
    ($ty:ident) => {
        impl_symmetric_binary_op!($ty, add, Add);
        impl_symmetric_binary_op!($ty, sub, Sub);
        impl_symmetric_binary_op!($ty, mul, Mul);
        impl_symmetric_binary_op!($ty, div, Div);

        impl_scalar_binary_op!($ty, add, Add);
        impl_scalar_binary_op!($ty, sub, Sub);
        impl_scalar_binary_op!($ty, mul, Mul);
        impl_scalar_binary_op!($ty, div, Div);
    };
}

impl_ops!(Vec3);

pub fn vec3<T: ScalarType>(
    x: impl IntoValue<Value = Scalar<T>>,
    y: impl IntoValue<Value = Scalar<T>>,
    z: impl IntoValue<Value = Scalar<T>>,
) -> Vec3<T> {
    builtin3("vec3", x, y, z)
}

pub fn vec4<T: ScalarType>(
    x: impl IntoValue<Value = Scalar<T>>,
    y: impl IntoValue<Value = Scalar<T>>,
    z: impl IntoValue<Value = Scalar<T>>,
    w: impl IntoValue<Value = Scalar<T>>,
) -> Vec4<T> {
    builtin4("vec4", x, y, z, w)
}
