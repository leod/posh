use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use crate::lang::{BinOp, Expr, ExprLit, Lit, ScalarType, Type};

use super::{binary, IntoValue, Trace, Value, ValueType};

pub trait ScalarValueType: Copy + Clone + ValueType + Into<Lit> {
    fn scalar_ty() -> ScalarType;
}

pub trait NumericValueType: ScalarValueType {}

#[derive(Debug, Copy, Clone)]
pub struct Scalar<T> {
    _phantom: PhantomData<T>,
    trace: Trace,
}

impl<T> Value for Scalar<T>
where
    T: ScalarValueType,
{
    type Type = T;

    fn from_trace(trace: Trace) -> Self {
        assert!(trace.expr().ty() == Self::Type::ty());

        Scalar {
            _phantom: PhantomData,
            trace,
        }
    }

    fn trace(&self) -> Trace {
        self.trace
    }
}

impl<T> Scalar<T>
where
    T: ScalarValueType,
{
    pub fn new(x: T) -> Self {
        Self::from_expr(Expr::Lit(ExprLit { lit: x.into() }))
    }

    pub fn eq<V>(&self, right: impl IntoValue<Value = V>) -> Bool
    where
        V: Value<Type = T>,
    {
        binary(*self, BinOp::Eq, right)
    }
}

impl<T> From<T> for Scalar<T>
where
    T: ScalarValueType,
{
    fn from(x: T) -> Self {
        Self::new(x)
    }
}

impl<T, Rhs> Add<Rhs> for Scalar<T>
where
    T: NumericValueType,
    Rhs: IntoValue<Value = Scalar<T>>,
{
    type Output = Self;

    fn add(self, right: Rhs) -> Self::Output {
        binary(self, BinOp::Add, right)
    }
}

impl<T, Rhs> Sub<Rhs> for Scalar<T>
where
    T: NumericValueType,
    Rhs: IntoValue<Value = Scalar<T>>,
{
    type Output = Self;

    fn sub(self, right: Rhs) -> Self::Output {
        binary(self, BinOp::Sub, right)
    }
}

impl<T, Rhs> Mul<Rhs> for Scalar<T>
where
    T: NumericValueType,
    Rhs: IntoValue<Value = Scalar<T>>,
{
    type Output = Self;

    fn mul(self, right: Rhs) -> Self::Output {
        binary(self, BinOp::Mul, right)
    }
}

impl<T, Rhs> Div<Rhs> for Scalar<T>
where
    T: NumericValueType,
    Rhs: IntoValue<Value = Scalar<T>>,
{
    type Output = Self;

    fn div(self, right: Rhs) -> Self::Output {
        binary(self, BinOp::Div, right)
    }
}

impl<T> IntoValue for T
where
    T: ScalarValueType,
{
    type Value = Scalar<T>;

    fn into_value(self) -> Self::Value {
        Scalar::new(self)
    }
}

impl<T> IntoValue for Scalar<T>
where
    T: ScalarValueType,
{
    type Value = Self;

    fn into_value(self) -> Self::Value {
        self
    }
}

impl ValueType for bool {
    type Value = Scalar<bool>;

    fn ty() -> Type {
        Type::Scalar(ScalarType::Bool)
    }
}

impl ValueType for u32 {
    type Value = Scalar<u32>;

    fn ty() -> Type {
        Type::Scalar(ScalarType::U32)
    }
}

impl ValueType for f32 {
    type Value = Scalar<f32>;

    fn ty() -> Type {
        Type::Scalar(ScalarType::F32)
    }
}

impl ScalarValueType for bool {
    fn scalar_ty() -> ScalarType {
        ScalarType::Bool
    }
}

impl ScalarValueType for u32 {
    fn scalar_ty() -> ScalarType {
        ScalarType::U32
    }
}

impl ScalarValueType for f32 {
    fn scalar_ty() -> ScalarType {
        ScalarType::F32
    }
}

impl NumericValueType for u32 {}
impl NumericValueType for f32 {}

pub type Bool = <bool as ValueType>::Value;
pub type U32 = <u32 as ValueType>::Value;
pub type F32 = <f32 as ValueType>::Value;
