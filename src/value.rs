use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use crate::{
    expr_reg::{self, ExprId},
    lang::{
        BinOp, Expr, ExprBinary, ExprCall, ExprCond, ExprLit, ExprVar, Func, Ident, Lit, Type, Var,
    },
};

pub trait ValueType {
    type Value;

    fn ty() -> Type;
}

pub type Fush<T> = <T as ValueType>::Value;

pub trait Value: Clone + Sized {
    type Type: ValueType;

    fn from_expr_id(expr_id: ExprId) -> Self;
    fn expr_id(&self) -> ExprId;

    fn ty(&self) -> Type {
        Self::Type::ty()
    }

    fn from_expr(expr: Expr) -> Self {
        Self::from_expr_id(expr_reg::put(expr))
    }

    fn expr(&self) -> Expr {
        expr_reg::get(self.expr_id())
    }

    fn map_expr(self, f: impl FnOnce(Expr) -> Expr) -> Self {
        Self::from_expr(f(self.expr()))
    }
}

pub trait ScalarType: Clone + ValueType + Into<Lit> {}

pub trait NumericType: ScalarType {}

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

impl ScalarType for bool {}
impl ScalarType for u32 {}
impl ScalarType for f32 {}

impl NumericType for u32 {}
impl NumericType for f32 {}

impl<T> Value for T
where
    T: ScalarType,
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
    T: ScalarType,
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
    T: ScalarType,
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
    T: ScalarType,
{
    fn from(x: T) -> Self {
        Self::from_expr(Expr::Lit(ExprLit { lit: x.into() }))
    }
}

impl<T, Rhs> Add<Rhs> for Scalar<T>
where
    T: NumericType,
    Rhs: Into<Scalar<T>>,
{
    type Output = Scalar<T>;

    fn add(self, rhs: Rhs) -> Scalar<T> {
        let left = Box::new(self.expr().clone());
        let right = Box::new(rhs.into().expr().clone());

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
    T: NumericType,
    Rhs: Into<Scalar<T>>,
{
    type Output = Scalar<T>;

    fn mul(self, rhs: Rhs) -> Scalar<T> {
        let left = Box::new(self.expr().clone());
        let right = Box::new(rhs.into().expr().clone());

        let expr = Expr::Binary(ExprBinary {
            left,
            op: BinOp::Mul,
            right,
        });

        Scalar::from_expr(expr)
    }
}

pub fn and(lhs: impl Into<Scalar<bool>>, rhs: impl Into<Scalar<bool>>) -> Scalar<bool> {
    let left = Box::new(lhs.into().expr().clone());
    let right = Box::new(rhs.into().expr().clone());

    let expr = Expr::Binary(ExprBinary {
        left,
        op: BinOp::And,
        right,
    });

    Scalar::from_expr(expr)
}

pub fn or(lhs: impl Into<Scalar<bool>>, rhs: impl Into<Scalar<bool>>) -> Scalar<bool> {
    let left = Box::new(lhs.into().expr().clone());
    let right = Box::new(rhs.into().expr().clone());

    let expr = Expr::Binary(ExprBinary {
        left,
        op: BinOp::Or,
        right,
    });

    Scalar::from_expr(expr)
}

pub fn func_call<V>(name: impl Into<String>, params: Vec<Var>, args: Vec<Expr>, result: V) -> V
where
    V: Value,
{
    assert!(params.len() == args.len());

    let func = Func::UserDefined {
        name: Ident::new(name),
        params,
        result: Box::new(result.expr().clone()),
    };
    let expr = Expr::Call(ExprCall { func, args });

    V::from_expr(expr)
}

pub fn eval<V>(init: V) -> V
where
    V: Value,
{
    let var = Var {
        ident: Ident::new("var"),
        ty: V::Type::ty(),
    };

    let init = Some(Box::new(init.expr()));

    let expr = Expr::Var(ExprVar { var, init });

    Value::from_expr(expr)
}

pub fn branch<B, V>(cond: B, true_value: impl Into<V>, false_value: impl Into<V>) -> V
where
    B: Into<Scalar<bool>>,
    V: Value,
{
    let cond = Box::new(cond.into().expr());
    let true_expr = Box::new(true_value.into().expr());
    let false_expr = Box::new(false_value.into().expr());

    let expr = Expr::Cond(ExprCond {
        cond,
        true_expr,
        false_expr,
    });

    V::from_expr(expr)
}

#[macro_export]
macro_rules! let_ {
    { $var:ident = $init:expr } => {
        let init = $init;
        let $var = $init.map_expr(|expr| $crate::lang::Expr::Var(
            $crate::lang::ExprVar {
                var: $crate::lang::Var {
                    ident: $crate::lang::Ident::new(std::stringify!($var)),
                    ty: Value::ty(&init),
                },
                init: Some(Box::new(expr)),
            },
        ));
    }
}
