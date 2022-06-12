use std::ops::{Add, Div, Mul, Sub};

use crate::{
    lang::{BinaryOp, BuiltInTy, Expr, Ident, Ty},
    IntoPosh, Value,
};

use super::{
    binary, builtin3, builtin4, field, scalar::NumericType, Lift, Scalar, ScalarType, Trace,
    TransparentValue, Type,
};

impl<T: ScalarType> Type for [T; 3] {
    fn ty() -> Ty {
        Ty::BuiltIn(BuiltInTy::Vec3(T::scalar_ty()))
    }
}

impl<T: ScalarType> Type for [T; 4] {
    fn ty() -> Ty {
        Ty::BuiltIn(BuiltInTy::Vec4(T::scalar_ty()))
    }
}

#[must_use]
#[derive(Debug, Copy, Clone)]
pub struct Vec3<'a, T> {
    trace: Trace<'a>,
    pub x: Scalar<'a, T>,
    pub y: Scalar<'a, T>,
    pub z: Scalar<'a, T>,
}

#[must_use]
#[derive(Debug, Clone, Copy)]
pub struct Vec4<'a, T> {
    trace: Trace<'a>,
    pub x: Scalar<'a, T>,
    pub y: Scalar<'a, T>,
    pub z: Scalar<'a, T>,
    pub w: Scalar<'a, T>,
}

impl<'a, T: ScalarType> Value<'a> for Vec3<'a, T> {
    type Type = [T; 3];

    fn from_ident(ident: Ident) -> Self {
        Self::from_trace(Trace::from_ident::<Self>(ident))
    }

    fn expr(&self) -> Expr {
        self.trace.expr()
    }
}

impl<'a, T: ScalarType> TransparentValue<'a> for Vec3<'a, T> {
    fn from_trace(trace: Trace<'a>) -> Self {
        assert!(trace.expr().ty() == <Self::Type as Type>::ty());

        Self {
            trace,
            x: field(trace, "x"),
            y: field(trace, "y"),
            z: field(trace, "z"),
        }
    }
}

impl<'a, T: ScalarType> Lift<'a> for [T; 3] {
    type Posh = Vec3<'a, T>;
}

impl<'a, T: ScalarType> Lift<'a> for [T; 4] {
    type Posh = Vec4<'a, T>;
}

impl<'a, T: ScalarType> IntoPosh<'a> for [T; 3] {
    fn into_posh(self) -> Self::Posh {
        vec3(self[0], self[1], self[2])
    }
}

impl<'a, T: ScalarType> IntoPosh<'a> for [T; 4] {
    fn into_posh(self) -> Self::Posh {
        vec4(self[0], self[1], self[2], self[3])
    }
}

impl<'a, T: ScalarType> Value<'a> for Vec4<'a, T> {
    type Type = [T; 4];

    fn from_ident(ident: Ident) -> Self {
        Self::from_trace(Trace::from_ident::<Self>(ident))
    }

    fn expr(&self) -> Expr {
        self.trace.expr()
    }
}

impl<'a, T: ScalarType> TransparentValue<'a> for Vec4<'a, T> {
    fn from_trace(trace: Trace<'a>) -> Self {
        assert!(trace.expr().ty() == <Self::Type as Type>::ty());

        Self {
            trace,
            x: field(trace, "x"),
            y: field(trace, "y"),
            z: field(trace, "z"),
            w: field(trace, "w"),
        }
    }
}

macro_rules! impl_symmetric_binary_op {
    ($ty:ident, $fn:ident, $op:ident) => {
        impl<'a, T> $op<$ty<'a, T>> for $ty<'a, T>
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
        impl<'a, T, Rhs> $op<Rhs> for $ty<'a, T>
        where
            T: NumericType,
            Rhs: IntoPosh<'a, Posh = Scalar<'a, T>>,
        {
            type Output = Self;

            fn $fn(self, right: Rhs) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl<'a, T> $op<$ty<'a, T>> for Scalar<'a, T>
        where
            T: NumericType,
        {
            type Output = $ty<'a, T>;

            fn $fn(self, right: $ty<'a, T>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl<'a> $op<$ty<'a, Self>> for f32 {
            type Output = $ty<'a, Self>;

            fn $fn(self, right: $ty<'a, Self>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl<'a> $op<$ty<'a, Self>> for i32 {
            type Output = $ty<'a, Self>;

            fn $fn(self, right: $ty<'a, Self>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl<'a> $op<$ty<'a, Self>> for u32 {
            type Output = $ty<'a, Self>;

            fn $fn(self, right: $ty<'a, Self>) -> Self::Output {
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

pub fn vec3<'a, T: ScalarType>(
    x: impl IntoPosh<'a, Posh = Scalar<'a, T>>,
    y: impl IntoPosh<'a, Posh = Scalar<'a, T>>,
    z: impl IntoPosh<'a, Posh = Scalar<'a, T>>,
) -> Vec3<'a, T> {
    builtin3("vec3", x, y, z)
}

pub fn vec4<'a, T: ScalarType>(
    x: impl IntoPosh<'a, Posh = Scalar<'a, T>>,
    y: impl IntoPosh<'a, Posh = Scalar<'a, T>>,
    z: impl IntoPosh<'a, Posh = Scalar<'a, T>>,
    w: impl IntoPosh<'a, Posh = Scalar<'a, T>>,
) -> Vec4<'a, T> {
    builtin4("vec4", x, y, z, w)
}
