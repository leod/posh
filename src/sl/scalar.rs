use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use super::{primitives::binary, Object, ToValue, Value};
use crate::{
    dag::{BaseTy, BinaryOp, Expr, Trace, Ty},
    Numeric, Primitive,
};

/// A scalar value in [`Posh`](crate::Posh).
#[derive(Debug, Copy, Clone)]
pub struct Scalar<T> {
    trace: Trace,
    _phantom: PhantomData<T>,
}

pub type F32 = Scalar<f32>;
pub type I32 = Scalar<i32>;
pub type U32 = Scalar<u32>;

impl<T: Primitive> Object for Scalar<T> {
    const TY: Ty = Ty::Base(BaseTy::Scalar(T::PRIMITIVE_TY));

    fn expr(&self) -> Rc<Expr> {
        self.trace.expr()
    }
}

impl<T: Primitive> Value for Scalar<T> {
    fn from_expr(expr: Expr) -> Self {
        assert!(expr.ty() == Self::TY);

        Self {
            trace: Trace::new(expr),
            _phantom: PhantomData,
        }
    }
}

impl<T: Primitive> Scalar<T> {
    pub fn new(x: T) -> Self {
        Self::from_expr(Expr::ScalarLiteral {
            ty: T::PRIMITIVE_TY,
            value: x.to_string(),
        })
    }

    pub fn eq(&self, right: impl ToValue<Value = Self>) -> Scalar<bool> {
        binary(*self, BinaryOp::Eq, right)
    }
}

impl Scalar<bool> {
    pub fn and(self, right: impl ToValue<Value = Self>) -> Self {
        binary(self, BinaryOp::And, right)
    }

    pub fn or(self, right: impl ToValue<Value = Self>) -> Self {
        binary(self, BinaryOp::And, right)
    }

    pub fn branch<V: Value>(self, yes: impl ToValue<Value = V>, no: impl ToValue<Value = V>) -> V {
        let ty = V::TY;
        let cond = self.expr();
        let yes = yes.to_value().expr();
        let no = no.to_value().expr();

        let expr = Expr::Branch { ty, cond, yes, no };

        V::from_expr(expr)
    }
}

/// Implement `Scalar<T> <op> impl ToPosh<Value = Scalar<T>>` for all
/// `T: Numeric`.
macro_rules! impl_binary_op_lhs {
    ($fn:ident, $op:ident) => {
        impl<T, Rhs> $op<Rhs> for Scalar<T>
        where
            T: Numeric,
            Rhs: ToValue<Value = Scalar<T>>,
        {
            type Output = Self;

            fn $fn(self, right: Rhs) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

/// Implement `$ty <op> Scalar<$ty>` where `$ty: Numeric`.
macro_rules! impl_binary_op_rhs {
    ($fn:ident, $op:ident, $ty:ty) => {
        impl $op<Scalar<Self>> for $ty {
            type Output = Scalar<Self>;

            fn $fn(self, right: Scalar<Self>) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

/// Implement a binary op for `Scalar<T>` for all `T: Numeric`.
macro_rules! impl_binary_op {
    ($fn:ident, $op:ident) => {
        impl_binary_op_lhs!($fn, $op);
        impl_binary_op_rhs!($fn, $op, i32);
        impl_binary_op_rhs!($fn, $op, u32);
        impl_binary_op_rhs!($fn, $op, f32);
    };
}

impl_binary_op!(add, Add);
impl_binary_op!(sub, Sub);
impl_binary_op!(mul, Mul);
impl_binary_op!(div, Div);

/// Implement conversions between `T` and `Scalar<T>` for all `T: Primitive`.
macro_rules! impl_conversions {
    ($ty:ty) => {
        impl ToValue for $ty {
            type Value = Scalar<$ty>;

            fn to_value(self) -> Self::Value {
                Scalar::new(self)
            }
        }

        impl From<$ty> for Scalar<$ty> {
            fn from(x: $ty) -> Self {
                x.to_value()
            }
        }
    };
}

impl_conversions!(bool);
impl_conversions!(i32);
impl_conversions!(u32);
impl_conversions!(f32);
