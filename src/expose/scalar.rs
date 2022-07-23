use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use sealed::sealed;

use crate::lang::{
    BinaryOp, BranchExpr, BuiltInTy, Expr, Ident, Literal, LiteralExpr, ScalarTy, Ty,
};

use super::{binary, Expose, FuncArg, IntoRep, Representative, Trace, Value};

/// A scalar type.
#[sealed]
pub trait ScalarType: Copy + Into<Literal> + IntoRep<Rep = Scalar<Self>> {
    fn scalar_ty() -> ScalarTy;
}

/// A numerical scalar type.
#[sealed]
pub trait NumType: ScalarType {}

/// Representative for scalars.
#[must_use]
#[derive(Debug, Copy, Clone)]
pub struct Scalar<T> {
    _phantom: PhantomData<T>,
    trace: Trace,
}

impl<T: ScalarType> Expose for Scalar<T> {
    type Rep = Self;
}

impl<T: ScalarType> Representative for Scalar<T> {}

impl<T: ScalarType> FuncArg for Scalar<T> {
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

impl<T: ScalarType> Value for Scalar<T> {
    fn from_trace(trace: Trace) -> Self {
        assert!(trace.expr().ty() == <Self::Rep as FuncArg>::ty());

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

    pub fn eq(&self, right: impl IntoRep<Rep = Self>) -> Scalar<bool> {
        binary(*self, BinaryOp::Eq, right)
    }
}

impl Scalar<bool> {
    pub fn and(self, right: impl IntoRep<Rep = Scalar<bool>>) -> Scalar<bool> {
        binary(self, BinaryOp::And, right)
    }

    pub fn or(self, right: impl IntoRep<Rep = Scalar<bool>>) -> Scalar<bool> {
        binary(self, BinaryOp::And, right)
    }

    pub fn branch<V: Value>(
        self,
        true_value: impl IntoRep<Rep = V>,
        false_value: impl IntoRep<Rep = V>,
    ) -> V {
        let cond = Rc::new(self.expr());
        let true_expr = Rc::new(true_value.into_rep().expr());
        let false_expr = Rc::new(false_value.into_rep().expr());

        let expr = Expr::Branch(BranchExpr {
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
            T: NumType,
            Rhs: IntoRep<Rep = Scalar<T>>,
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
        #[sealed]
        impl ScalarType for $ty {
            fn scalar_ty() -> ScalarTy {
                ScalarTy::$name
            }
        }

        impl Expose for $ty {
            type Rep = Scalar<$ty>;
        }

        impl IntoRep for $ty {
            fn into_rep(self) -> Self::Rep {
                Scalar::new(self)
            }
        }

        impl From<$ty> for Scalar<$ty> {
            fn from(x: $ty) -> Self {
                x.into_rep()
            }
        }
    };
}

impl_scalar!(f32, F32);
impl_scalar!(i32, I32);
impl_scalar!(u32, U32);
impl_scalar!(bool, Bool);

#[sealed]
impl NumType for f32 {}

#[sealed]
impl NumType for i32 {}

#[sealed]
impl NumType for u32 {}
