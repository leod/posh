use std::ops::{Add, Mul};

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

impl<T> Add<Vec3<T>> for Vec3<T>
where
    T: NumericValueType,
{
    type Output = Self;

    fn add(self, right: Self) -> Self::Output {
        binary(self, BinOp::Add, right)
    }
}

impl<T> Mul<Vec3<T>> for Vec3<T>
where
    T: NumericValueType,
{
    type Output = Self;

    fn mul(self, right: Self) -> Self::Output {
        binary(self, BinOp::Mul, right)
    }
}

impl<T, Rhs> Add<Rhs> for Vec3<T>
where
    T: NumericValueType,
    Rhs: IntoValue<Value = Scalar<T>>,
{
    type Output = Self;

    fn add(self, right: Rhs) -> Self::Output {
        binary(self, BinOp::Add, right)
    }
}

impl<T, Rhs> Mul<Rhs> for Vec3<T>
where
    T: NumericValueType,
    Rhs: IntoValue<Value = Scalar<T>>,
{
    type Output = Self;

    fn mul(self, right: Rhs) -> Self::Output {
        binary(self, BinOp::Mul, right)
    }
}

impl<T> IntoValue for Vec3<T>
where
    T: ScalarValueType,
{
    type Value = Self;

    fn into_value(self) -> Self::Value {
        self
    }
}

impl Vec3<f32> {
    pub fn normalize(self) -> F32 {
        builtin1("normalize", self)
    }
}

pub fn vec3<T: ScalarValueType>(
    x: impl IntoValue<Value = Scalar<T>>,
    y: impl IntoValue<Value = Scalar<T>>,
    z: impl IntoValue<Value = Scalar<T>>,
) -> Vec3<T> {
    builtin3("vec3", x, y, z)
}
