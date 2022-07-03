use std::ops::{Add, Div, Mul, Sub};

use crate::{
    lang::{BinaryOp, BuiltInTy, Expr, Ident, Ty},
    IntoPosh, ValueBase,
};

use super::{
    binary, builtin3, builtin4, field, scalar::NumericType, Lift, Scalar, ScalarType, Trace, Value,
};

#[must_use]
#[derive(Debug, Copy, Clone)]
pub struct Vec3<T> {
    trace: Trace,
    pub x: Scalar<T>,
    pub y: Scalar<T>,
    pub z: Scalar<T>,
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

impl<T: ScalarType> ValueBase for Vec3<T> {
    fn ty() -> Ty {
        Ty::BuiltIn(BuiltInTy::Vec3(T::scalar_ty()))
    }

    fn expr(&self) -> Expr {
        self.trace.expr()
    }

    fn from_ident(ident: Ident) -> Self {
        Self::from_trace(Trace::from_ident::<Self>(ident))
    }
}

impl<T: ScalarType> ValueBase for Vec4<T> {
    fn ty() -> Ty {
        Ty::BuiltIn(BuiltInTy::Vec4(T::scalar_ty()))
    }

    fn from_ident(ident: Ident) -> Self {
        Self::from_trace(Trace::from_ident::<Self>(ident))
    }

    fn expr(&self) -> Expr {
        self.trace.expr()
    }
}

impl<T: ScalarType> Value for Vec3<T> {
    fn from_trace(trace: Trace) -> Self {
        assert!(trace.expr().ty() == <Self::Type as ValueBase>::ty());

        Self {
            trace,
            x: field(trace, "x"),
            y: field(trace, "y"),
            z: field(trace, "z"),
        }
    }
}

impl<T: ScalarType> Value for Vec4<T> {
    fn from_trace(trace: Trace) -> Self {
        assert!(trace.expr().ty() == <Self::Type as ValueBase>::ty());

        Self {
            trace,
            x: field(trace, "x"),
            y: field(trace, "y"),
            z: field(trace, "z"),
            w: field(trace, "w"),
        }
    }
}

impl<T: ScalarType> Lift for [T; 3] {
    type Type = Vec3<T>;
}

impl<T: ScalarType> Lift for [T; 4] {
    type Type = Vec4<T>;
}

impl<T: ScalarType> Lift for Vec3<T> {
    type Type = Self;
}

impl<T: ScalarType> Lift for Vec4<T> {
    type Type = Self;
}

impl<T: ScalarType> IntoPosh for [T; 3] {
    fn into_posh(self) -> Self::Type {
        vec3(self[0], self[1], self[2])
    }
}

impl<T: ScalarType> IntoPosh for [T; 4] {
    fn into_posh(self) -> Self::Type {
        vec4(self[0], self[1], self[2], self[3])
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
            Rhs: IntoPosh<Type = Scalar<T>>,
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
    x: impl IntoPosh<Type = Scalar<T>>,
    y: impl IntoPosh<Type = Scalar<T>>,
    z: impl IntoPosh<Type = Scalar<T>>,
) -> Vec3<T> {
    builtin3("vec3", x, y, z)
}

pub fn vec4<T: ScalarType>(
    x: impl IntoPosh<Type = Scalar<T>>,
    y: impl IntoPosh<Type = Scalar<T>>,
    z: impl IntoPosh<Type = Scalar<T>>,
    w: impl IntoPosh<Type = Scalar<T>>,
) -> Vec4<T> {
    builtin4("vec4", x, y, z, w)
}
