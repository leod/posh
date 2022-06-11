use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use crate::lang::{BinaryOp, BuiltInTy, Expr, Literal, LiteralExpr, ScalarTy, TernaryExpr, Ty};

use super::{binary, BuiltIn, HasValue, IntoValue, Trace, Transparent, Type, Value};

pub trait ScalarType: BuiltIn + Copy + Into<Literal> + IntoValue<Value = Scalar<Self>> {
    fn scalar_ty() -> ScalarTy;
}

pub trait NumericType: ScalarType {}

#[must_use]
#[derive(Debug, Copy, Clone)]
pub struct Scalar<T> {
    _phantom: PhantomData<T>,
    trace: Trace,
}

impl<T> Value for Scalar<T>
where
    T: ScalarType,
{
    type Type = T;

    fn from_trace(trace: Trace) -> Self {
        assert!(trace.expr().ty() == <Self::Type as Type>::ty());

        Scalar {
            _phantom: PhantomData,
            trace,
        }
    }

    fn expr(&self) -> Expr {
        self.trace.expr()
    }
}

impl<T> Type for T
where
    T: ScalarType,
{
    fn ty() -> Ty {
        Ty::BuiltIn(Self::built_in_ty())
    }
}

impl<T> Transparent for T
where
    T: ScalarType,
{
    fn transparent() {}
}

impl<T> Scalar<T>
where
    T: ScalarType,
{
    pub fn new(x: T) -> Self {
        Self::from_expr(Expr::Literal(LiteralExpr { literal: x.into() }))
    }

    pub fn eq<V>(&self, right: impl IntoValue<Value = V>) -> Bool
    where
        V: Value<Type = T>,
    {
        binary(*self, BinaryOp::Eq, right)
    }
}

impl Bool {
    pub fn and(self, right: impl IntoValue<Value = Bool>) -> Bool {
        binary(self, BinaryOp::And, right)
    }

    pub fn or(self, right: impl IntoValue<Value = Bool>) -> Bool {
        binary(self, BinaryOp::And, right)
    }

    pub fn ternary<V>(
        self,
        true_value: impl IntoValue<Value = V>,
        false_value: impl IntoValue<Value = V>,
    ) -> V
    where
        V: Value,
        V::Type: Transparent,
    {
        let cond = Rc::new(self.expr());
        let true_expr = Rc::new(true_value.into_value().expr());
        let false_expr = Rc::new(false_value.into_value().expr());

        let expr = Expr::Ternary(TernaryExpr {
            cond,
            true_expr,
            false_expr,
        });

        V::from_expr(expr)
    }
}

macro_rules! impl_binary_op {
    ($fn:ident, $op:ident) => {
        impl<T, Rhs> $op<Rhs> for Scalar<T>
        where
            T: NumericType,
            Rhs: IntoValue<Value = Scalar<T>>,
        {
            type Output = Self;

            fn $fn(self, right: Rhs) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<Scalar<Self>> for f32 {
            type Output = Scalar<Self>;

            fn $fn(self, right: Scalar<Self>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<Scalar<Self>> for i32 {
            type Output = Scalar<Self>;

            fn $fn(self, right: Scalar<Self>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl $op<Scalar<Self>> for u32 {
            type Output = Scalar<Self>;

            fn $fn(self, right: Scalar<Self>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
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
        impl BuiltIn for $ty {
            fn built_in_ty() -> BuiltInTy {
                BuiltInTy::Scalar(Self::scalar_ty())
            }
        }

        impl ScalarType for $ty {
            fn scalar_ty() -> ScalarTy {
                ScalarTy::$name
            }
        }

        impl HasValue for $ty {
            type Value = Scalar<$ty>;
        }

        impl IntoValue for $ty {
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

impl NumericType for f32 {}
impl NumericType for i32 {}
impl NumericType for u32 {}
