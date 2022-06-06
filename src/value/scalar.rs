use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};

use crate::lang::{BinOp, Expr, ExprLit, Lit, ScalarType, Type, TypeBuiltIn};

use super::{binary, BuiltInValueType, IntoValue, Trace, Value, ValueType};

pub trait ScalarValueType: BuiltInValueType + Copy + Into<Lit> {
    fn scalar_ty() -> ScalarType;
}

pub trait NumericValueType: ScalarValueType {}

#[must_use]
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

macro_rules! impl_binary_op {
    ($fn:ident, $op:ident) => {
        impl<T, Rhs> $op<Rhs> for Scalar<T>
        where
            T: NumericValueType,
            Rhs: IntoValue<Value = Scalar<T>>,
        {
            type Output = Self;

            fn $fn(self, right: Rhs) -> Self::Output {
                binary(self, BinOp::$op, right)
            }
        }

        impl $op<Scalar<Self>> for f32 {
            type Output = Scalar<Self>;

            fn $fn(self, right: Scalar<Self>) -> Self::Output {
                binary(self, BinOp::$op, right)
            }
        }

        impl $op<Scalar<Self>> for i32 {
            type Output = Scalar<Self>;

            fn $fn(self, right: Scalar<Self>) -> Self::Output {
                binary(self, BinOp::$op, right)
            }
        }

        impl $op<Scalar<Self>> for u32 {
            type Output = Scalar<Self>;

            fn $fn(self, right: Scalar<Self>) -> Self::Output {
                binary(self, BinOp::$op, right)
            }
        }
    };
}

impl_binary_op!(add, Add);
impl_binary_op!(sub, Sub);
impl_binary_op!(mul, Mul);
impl_binary_op!(div, Div);

macro_rules! impl_scalar {
    ($ty:ty, $name:ident) => {
        impl ValueType for $ty {
            type Value = Scalar<$ty>;

            fn ty() -> Type {
                Type::BuiltIn(Self::built_in_ty())
            }
        }

        impl BuiltInValueType for $ty {
            fn built_in_ty() -> TypeBuiltIn {
                TypeBuiltIn::Scalar(Self::scalar_ty())
            }
        }

        impl ScalarValueType for $ty {
            fn scalar_ty() -> ScalarType {
                ScalarType::$name
            }
        }

        impl IntoValue for $ty {
            type Value = Scalar<$ty>;

            fn into_value(self) -> Self::Value {
                Scalar::new(self)
            }
        }

        pub type $name = Scalar<$ty>;
    };
}

impl_scalar!(f32, F32);
impl_scalar!(i32, I32);
impl_scalar!(u32, U32);
impl_scalar!(bool, Bool);

impl NumericValueType for f32 {}
impl NumericValueType for i32 {}
impl NumericValueType for u32 {}
