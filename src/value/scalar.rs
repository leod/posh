use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use crate::lang::{
    BinaryOp, BuiltInTy, Expr, Ident, Literal, LiteralExpr, ScalarTy, TernaryExpr, Ty,
};

use super::{binary, IntoPosh, Lift, Trace, TransparentValue, Type, Value};

pub trait ScalarType:
    Type + Copy + Into<Literal> + for<'a> IntoPosh<'a, Posh = Scalar<'a, Self>>
{
    fn scalar_ty() -> ScalarTy;
}

pub trait NumericType: ScalarType {}

impl<T> Type for T
where
    T: ScalarType,
{
    fn ty() -> Ty {
        Ty::BuiltIn(BuiltInTy::Scalar(T::scalar_ty()))
    }
}

#[must_use]
#[derive(Debug, Copy, Clone)]
pub struct Scalar<'a, T> {
    _phantom: PhantomData<T>,
    trace: Trace<'a>,
}

impl<'a, T> Value<'a> for Scalar<'a, T>
where
    T: ScalarType,
{
    type Type = T;

    fn from_ident(ident: Ident) -> Self {
        Self::from_trace(Trace::from_ident::<Self>(ident))
    }

    fn expr(&self) -> Expr {
        self.trace.expr()
    }
}

impl<'a, T> TransparentValue<'a> for Scalar<'a, T>
where
    T: ScalarType,
{
    fn from_trace(trace: Trace<'a>) -> Self {
        assert!(trace.expr().ty() == <Self::Type as Type>::ty());

        Scalar {
            _phantom: PhantomData,
            trace,
        }
    }
}

impl<'a, T> Scalar<'a, T>
where
    T: ScalarType,
{
    pub fn new(x: T) -> Self {
        Self::from_expr(Expr::Literal(LiteralExpr { literal: x.into() }))
    }

    pub fn eq<V>(&self, right: impl IntoPosh<'a, Posh = V>) -> Bool
    where
        V: Value<'a, Type = T>,
    {
        binary(*self, BinaryOp::Eq, right)
    }
}

impl<'a> Bool<'a> {
    pub fn and(self, right: impl IntoPosh<'a, Posh = Bool<'a>>) -> Bool<'a> {
        binary(self, BinaryOp::And, right)
    }

    pub fn or(self, right: impl IntoPosh<'a, Posh = Bool<'a>>) -> Bool<'a> {
        binary(self, BinaryOp::And, right)
    }

    pub fn ternary<V: TransparentValue<'a>>(
        self,
        true_value: impl IntoPosh<'a, Posh = V>,
        false_value: impl IntoPosh<'a, Posh = V>,
    ) -> V {
        let cond = Rc::new(self.expr());
        let true_expr = Rc::new(true_value.into_posh().expr());
        let false_expr = Rc::new(false_value.into_posh().expr());

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
        impl<'a, T, Rhs> $op<Rhs> for Scalar<'a, T>
        where
            T: NumericType,
            Rhs: IntoPosh<'a, Posh = Scalar<'a, T>>,
        {
            type Output = Self;

            fn $fn(self, right: Rhs) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl<'a> $op<Scalar<'a, Self>> for f32 {
            type Output = Scalar<'a, Self>;

            fn $fn(self, right: Scalar<'a, Self>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl<'a> $op<Scalar<'a, Self>> for i32 {
            type Output = Scalar<'a, Self>;

            fn $fn(self, right: Scalar<'a, Self>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }

        impl<'a> $op<Scalar<'a, Self>> for u32 {
            type Output = Scalar<'a, Self>;

            fn $fn(self, right: Scalar<'a, Self>) -> Self::Output {
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
        impl ScalarType for $ty {
            fn scalar_ty() -> ScalarTy {
                ScalarTy::$name
            }
        }

        impl<'a> Lift<'a> for $ty {
            type Posh = Scalar<'a, $ty>;
        }

        impl<'a> IntoPosh<'a> for $ty {
            fn into_posh(self) -> Self::Posh {
                Scalar::new(self)
            }
        }

        pub type $name<'a> = Scalar<'a, $ty>;
    };
}

impl_scalar!(f32, F32);
impl_scalar!(i32, I32);
impl_scalar!(u32, U32);
impl_scalar!(bool, Bool);

impl NumericType for f32 {}
impl NumericType for i32 {}
impl NumericType for u32 {}
