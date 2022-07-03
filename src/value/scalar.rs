use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use crate::lang::{
    BinaryOp, BuiltInTy, Expr, Ident, Literal, LiteralExpr, ScalarTy, TernaryExpr, Ty,
};

use super::{binary, ConstructibleVal, IntoVal, Trace, Type, TypedVal, Val};

pub trait ScalarType: Copy + Into<Literal> + IntoVal<Val = Scalar<Self>> {
    fn scalar_ty() -> ScalarTy;
}

pub trait NumericType: ScalarType {}

impl<T: ScalarType> Type for Scalar<T> {
    type Val = Self;
}

#[must_use]
#[derive(Debug, Copy, Clone)]
pub struct Scalar<T> {
    _phantom: PhantomData<T>,
    trace: Trace,
}

impl<T: ScalarType> Val for Scalar<T> {}

impl<T: ScalarType> TypedVal for Scalar<T> {
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

impl<T: ScalarType> ConstructibleVal for Scalar<T> {
    fn from_trace(trace: Trace) -> Self {
        assert!(trace.expr().ty() == <Self::Val as TypedVal>::ty());

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

    pub fn eq(&self, right: impl IntoVal<Val = Self>) -> Bool {
        binary(*self, BinaryOp::Eq, right)
    }
}

impl Bool {
    pub fn and(self, right: impl IntoVal<Val = Bool>) -> Bool {
        binary(self, BinaryOp::And, right)
    }

    pub fn or(self, right: impl IntoVal<Val = Bool>) -> Bool {
        binary(self, BinaryOp::And, right)
    }

    pub fn ternary<V: ConstructibleVal>(
        self,
        true_value: impl IntoVal<Val = V>,
        false_value: impl IntoVal<Val = V>,
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
            Rhs: IntoVal<Val = Scalar<T>>,
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

        impl Type for $ty {
            type Val = Scalar<$ty>;
        }

        impl IntoVal for $ty {
            fn into_val(self) -> Self::Val {
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
