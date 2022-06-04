use std::ops::{Add, Div, Mul, Sub};

use crate::{
    lang::{BinOp, Type},
    value::primitives::field,
    IntoValue, Value, F32,
};

use super::{
    binary, builtin1, builtin3, scalar::NumericValueType, Scalar, ScalarValueType, Trace, ValueType,
};

impl<T: ScalarValueType> ValueType for [T; 3] {
    type Value = Vec3<T>;

    fn ty() -> Type {
        Type::Vec3(T::scalar_ty())
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

impl<T: ScalarValueType> Value for Vec3<T> {
    type Type = [T; 3];

    fn from_trace(trace: Trace) -> Self {
        assert!(trace.expr().ty() == Self::Type::ty());

        Self {
            trace,
            x: field(trace, "x"),
            y: field(trace, "y"),
            z: field(trace, "z"),
        }
    }

    fn trace(&self) -> Trace {
        self.trace
    }
}

macro_rules! impl_symmetric_binary_op {
    ($ty:ident, $fn:ident, $op:ident) => {
        impl<T> $op<$ty<T>> for $ty<T>
        where
            T: NumericValueType,
        {
            type Output = Self;

            fn $fn(self, right: Self) -> Self::Output {
                binary(self, BinOp::$op, right)
            }
        }
    };
}

macro_rules! impl_scalar_binary_op {
    ($ty:ident, $fn:ident, $op:ident) => {
        impl<T, Rhs> $op<Rhs> for $ty<T>
        where
            T: NumericValueType,
            Rhs: IntoValue<Value = Scalar<T>>,
        {
            type Output = Self;

            fn $fn(self, right: Rhs) -> Self::Output {
                binary(self, BinOp::$op, right)
            }
        }

        impl<T> $op<$ty<T>> for Scalar<T>
        where
            T: NumericValueType,
        {
            type Output = $ty<T>;

            fn $fn(self, right: $ty<T>) -> Self::Output {
                binary(self, BinOp::$op, right)
            }
        }

        impl $op<$ty<Self>> for f32 {
            type Output = $ty<Self>;

            fn $fn(self, right: $ty<Self>) -> Self::Output {
                binary(self, BinOp::$op, right)
            }
        }

        impl $op<$ty<Self>> for i32 {
            type Output = $ty<Self>;

            fn $fn(self, right: $ty<Self>) -> Self::Output {
                binary(self, BinOp::$op, right)
            }
        }

        impl $op<$ty<Self>> for u32 {
            type Output = $ty<Self>;

            fn $fn(self, right: $ty<Self>) -> Self::Output {
                binary(self, BinOp::$op, right)
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

pub trait GenValue: Value + Sized {
    fn normalize(self) -> Self {
        builtin1("normalize", self)
    }
}

impl GenValue for F32 {}
impl GenValue for Vec3<f32> {}

pub fn vec3<T: ScalarValueType>(
    x: impl IntoValue<Value = Scalar<T>>,
    y: impl IntoValue<Value = Scalar<T>>,
    z: impl IntoValue<Value = Scalar<T>>,
) -> Vec3<T> {
    builtin3("vec3", x, y, z)
}
