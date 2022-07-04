use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use crate::lang::{
    BinaryOp, BuiltInTy, Expr, Ident, Literal, LiteralExpr, ScalarTy, TernaryExpr, Ty,
};

use super::{binary, Constructible, IntoVal, Lift, Trace, Value, ValueBase};

pub trait ScalarType: Copy + Into<Literal> + IntoVal<Value = Scalar<Self>> {
    fn scalar_ty() -> ScalarTy;
}

pub trait NumericType: ScalarType {}

impl<T: ScalarType> Lift for Scalar<T> {
    type Value = Self;
}

#[must_use]
#[derive(Debug, Copy, Clone)]
pub struct Scalar<T> {
    _phantom: PhantomData<T>,
    trace: Trace,
}

impl<T: ScalarType> ValueBase for Scalar<T> {}

impl<T: ScalarType> Value for Scalar<T> {
    fn ty() -> Ty {
        Ty::BuiltIn(BuiltInTy::Scalar(T::scalar_ty()))
    }

    fn expr(&self) -> Expr {
        self.trace.expr()
    }

    fn from_ident(ident: Ident) -> Self {
        Self::from_trace(Trace::from_ident::<Self>(ident))
    }
}

impl<T: ScalarType> Constructible for Scalar<T> {
    fn from_trace(trace: Trace) -> Self {
        assert!(trace.expr().ty() == <Self::Value as Value>::ty());

        Scalar {
            _phantom: PhantomData,
            trace,
        }
    }
}

impl<T> Scalar<T>
where
    T: ScalarType,
{
    pub fn new(x: T) -> Self {
        Self::from_expr(Expr::Literal(LiteralExpr { literal: x.into() }))
    }

    pub fn eq(&self, right: impl IntoVal<Value = Self>) -> Bool {
        binary(*self, BinaryOp::Eq, right)
    }
}

impl Bool {
    pub fn and(self, right: impl IntoVal<Value = Bool>) -> Bool {
        binary(self, BinaryOp::And, right)
    }

    pub fn or(self, right: impl IntoVal<Value = Bool>) -> Bool {
        binary(self, BinaryOp::And, right)
    }

    pub fn ternary<V: Constructible>(
        self,
        true_value: impl IntoVal<Value = V>,
        false_value: impl IntoVal<Value = V>,
    ) -> V {
        let cond = Rc::new(self.expr());
        let true_expr = Rc::new(true_value.into_val().expr());
        let false_expr = Rc::new(false_value.into_val().expr());

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
            Rhs: IntoVal<Value = Scalar<T>>,
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
        impl ScalarType for $ty {
            fn scalar_ty() -> ScalarTy {
                ScalarTy::$name
            }
        }

        impl Lift for $ty {
            type Value = Scalar<$ty>;
        }

        impl IntoVal for $ty {
            fn into_val(self) -> Self::Value {
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
