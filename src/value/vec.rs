use std::ops::{Add, Div, Mul, Sub};

use crate::lang::{BinaryOp, BuiltInTy, Expr, Ident, Ty};

use super::{
    binary, builtin3, builtin4, field, scalar::NumericType, ConstructibleVal, IntoVal, Scalar,
    ScalarType, Trace, Type, TypedVal, Val,
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

impl<T: ScalarType> Val for Vec3<T> {}

impl<T: ScalarType> TypedVal for Vec3<T> {
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

impl<T: ScalarType> Val for Vec4<T> {}

impl<T: ScalarType> TypedVal for Vec4<T> {
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

impl<T: ScalarType> ConstructibleVal for Vec3<T> {
    fn from_trace(trace: Trace) -> Self {
        assert!(trace.expr().ty() == <Self::Val as TypedVal>::ty());

        Self {
            trace,
            x: field(trace, "x"),
            y: field(trace, "y"),
            z: field(trace, "z"),
        }
    }
}

impl<T: ScalarType> ConstructibleVal for Vec4<T> {
    fn from_trace(trace: Trace) -> Self {
        assert!(trace.expr().ty() == <Self::Val as TypedVal>::ty());

        Self {
            trace,
            x: field(trace, "x"),
            y: field(trace, "y"),
            z: field(trace, "z"),
            w: field(trace, "w"),
        }
    }
}

impl<T: ScalarType> Type for [T; 3] {
    type Val = Vec3<T>;
}

impl<T: ScalarType> Type for [T; 4] {
    type Val = Vec4<T>;
}

impl<T: ScalarType> Type for Vec3<T> {
    type Val = Self;
}

impl<T: ScalarType> Type for Vec4<T> {
    type Val = Self;
}

impl<T: ScalarType> IntoVal for [T; 3] {
    fn into_val(self) -> Self::Val {
        vec3(self[0], self[1], self[2])
    }
}

impl<T: ScalarType> IntoVal for [T; 4] {
    fn into_val(self) -> Self::Val {
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
            Rhs: IntoVal<Val = Scalar<T>>,
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
    x: impl IntoVal<Val = Scalar<T>>,
    y: impl IntoVal<Val = Scalar<T>>,
    z: impl IntoVal<Val = Scalar<T>>,
) -> Vec3<T> {
    builtin3("vec3", x, y, z)
}

pub fn vec4<T: ScalarType>(
    x: impl IntoVal<Val = Scalar<T>>,
    y: impl IntoVal<Val = Scalar<T>>,
    z: impl IntoVal<Val = Scalar<T>>,
    w: impl IntoVal<Val = Scalar<T>>,
) -> Vec4<T> {
    builtin4("vec4", x, y, z, w)
}
