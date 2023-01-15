use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use super::{
    primitives::{binary, built_in_1, value_arg},
    Object, ToValue, Value,
};
use crate::{
    dag::{BaseType, BinaryOp, Expr, Trace, Type},
    Numeric, Primitive,
};

/// A scalar value in the shading language.
#[derive(Debug, Copy, Clone)]
pub struct Scalar<T> {
    trace: Trace,
    _phantom: PhantomData<T>,
}

/// A boolean value in the shading language.
pub type Bool = Scalar<bool>;

/// A floating-point value in the shading language.
pub type F32 = Scalar<f32>;

/// A signed integer value in the shading language.
pub type I32 = Scalar<i32>;

/// An unsigned integer value in the shading language.
pub type U32 = Scalar<u32>;

impl<T: Primitive> Default for Scalar<T> {
    fn default() -> Self {
        T::default().to_value()
    }
}

impl<T: Primitive> Object for Scalar<T> {
    fn ty() -> Type {
        Type::Base(BaseType::Scalar(T::PRIMITIVE_TYPE))
    }

    fn expr(&self) -> Rc<Expr> {
        self.trace.expr()
    }

    fn from_arg(path: &str) -> Self {
        value_arg(path)
    }
}

impl<T: Primitive> Value for Scalar<T> {
    fn from_expr(expr: Expr) -> Self {
        assert!(expr.ty() == Self::ty());

        Self {
            trace: Trace::new(expr),
            _phantom: PhantomData,
        }
    }
}

impl<T: Primitive> ToValue for T {
    type Output = Scalar<T>;

    fn to_value(self) -> Self::Output {
        Scalar::new(self)
    }
}

impl<T: Primitive> ToValue for Scalar<T> {
    type Output = Self;

    fn to_value(self) -> Self::Output {
        self
    }
}

impl<T: Primitive> From<T> for Scalar<T> {
    fn from(x: T) -> Self {
        x.to_value()
    }
}

impl<T: Primitive> Scalar<T> {
    pub fn new(x: T) -> Self {
        Self::from_expr(Expr::ScalarLiteral {
            ty: T::PRIMITIVE_TYPE,
            value: x.to_string(),
        })
    }

    pub fn eq(&self, right: impl ToValue<Output = Self>) -> Scalar<bool> {
        binary(*self, BinaryOp::Eq, right)
    }

    pub fn cast<U: Primitive>(self) -> Scalar<U> {
        built_in_1(&format!("{}", U::PRIMITIVE_TYPE), self)
    }
}

impl Scalar<bool> {
    pub fn and(self, right: impl ToValue<Output = Self>) -> Self {
        binary(self, BinaryOp::And, right)
    }

    pub fn or(self, right: impl ToValue<Output = Self>) -> Self {
        binary(self, BinaryOp::And, right)
    }

    pub fn branch<V: Value>(
        self,
        yes: impl ToValue<Output = V>,
        no: impl ToValue<Output = V>,
    ) -> V {
        let ty = V::ty();
        let cond = self.expr();
        let yes = yes.to_value().expr();
        let no = no.to_value().expr();

        let expr = Expr::Branch { ty, cond, yes, no };

        V::from_expr(expr)
    }
}

// Implements `Scalar<T> <op> impl ToValue<Output = Scalar<T>>` for all `T:
// Numeric`.
macro_rules! impl_binary_op_lhs {
    ($fn:ident, $op:ident) => {
        impl<T, Rhs> $op<Rhs> for Scalar<T>
        where
            T: Numeric,
            Rhs: ToValue<Output = Scalar<T>>,
        {
            type Output = Self;

            fn $fn(self, right: Rhs) -> Self::Output {
                binary(self, BinaryOp::$op, right)
            }
        }
    };
}

// Implements `$ty <op> Scalar<$ty>` where `$ty: Numeric`.
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

// Implements a binary op for `Scalar<T>` for all `T: Numeric`.
macro_rules! impl_binary_op {
    ($fn:ident, $op:ident) => {
        impl_binary_op_lhs!($fn, $op);
        impl_binary_op_rhs!($fn, $op, i32);
        impl_binary_op_rhs!($fn, $op, u32);
        impl_binary_op_rhs!($fn, $op, f32);
    };
}

impl_binary_op!(add, Add);
impl_binary_op!(div, Div);
impl_binary_op!(mul, Mul);
impl_binary_op!(sub, Sub);
