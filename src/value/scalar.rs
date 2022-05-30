use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use crate::{
    expr_reg::ExprId,
    lang::{BinOp, Expr, ExprBinary, ExprLit, Lit, Type},
};

use super::{Value, ValueType};

pub trait ScalarValueType: Clone + ValueType + Into<Lit> {}

pub trait NumericValueType: ScalarValueType {}

impl<T> Value for T
where
    T: ScalarValueType,
{
    type Type = T;

    fn from_expr_id(_: ExprId) -> Self {
        unimplemented!();
    }

    fn expr_id(&self) -> ExprId {
        Scalar::from(self.clone()).expr_id
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Scalar<T> {
    _phantom: PhantomData<T>,
    expr_id: ExprId,
}

impl<T> Value for Scalar<T>
where
    T: ScalarValueType,
{
    type Type = T;

    fn from_expr_id(expr_id: ExprId) -> Self {
        Scalar {
            _phantom: PhantomData,
            expr_id,
        }
    }

    fn expr_id(&self) -> ExprId {
        self.expr_id
    }
}

impl<T> Scalar<T>
where
    T: ScalarValueType,
{
    pub fn eq<V>(&self, rhs: V) -> Scalar<bool>
    where
        V: Value<Type = T>,
    {
        let left = Box::new(self.expr());
        let right = Box::new(rhs.expr());

        let expr = Expr::Binary(ExprBinary {
            left,
            op: BinOp::Eq,
            right,
        });

        Scalar::from_expr(expr)
    }
}

impl<T> From<T> for Scalar<T>
where
    T: ScalarValueType,
{
    fn from(x: T) -> Self {
        Self::from_expr(Expr::Lit(ExprLit { lit: x.into() }))
    }
}

impl<T, Rhs> Add<Rhs> for Scalar<T>
where
    T: NumericValueType,
    Rhs: Into<Scalar<T>>,
{
    type Output = Scalar<T>;

    fn add(self, rhs: Rhs) -> Self::Output {
        let left = Box::new(self.expr());
        let right = Box::new(rhs.into().expr());

        let expr = Expr::Binary(ExprBinary {
            left,
            op: BinOp::Add,
            right,
        });

        Scalar::from_expr(expr)
    }
}

impl<T, Rhs> Mul<Rhs> for Scalar<T>
where
    T: NumericValueType,
    Rhs: Into<Scalar<T>>,
{
    type Output = Scalar<T>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        let left = Box::new(self.expr());
        let right = Box::new(rhs.into().expr());

        let expr = Expr::Binary(ExprBinary {
            left,
            op: BinOp::Mul,
            right,
        });

        Scalar::from_expr(expr)
    }
}

impl ValueType for bool {
    type Value = Scalar<bool>;

    fn ty() -> Type {
        Type::U32
    }
}

impl ValueType for u32 {
    type Value = Scalar<u32>;

    fn ty() -> Type {
        Type::U32
    }
}

impl ValueType for f32 {
    type Value = Scalar<f32>;

    fn ty() -> Type {
        Type::F32
    }
}

impl ScalarValueType for bool {}
impl ScalarValueType for u32 {}
impl ScalarValueType for f32 {}

impl NumericValueType for u32 {}
impl NumericValueType for f32 {}

pub type Bool = Scalar<bool>;
pub type U32 = Scalar<u32>;
pub type F32 = Scalar<f32>;
