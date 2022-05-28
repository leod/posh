use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use crate::{
    expr_reg::{self, ExprId},
    lang::{self, Expr, Type},
};

pub trait ValueType {
    fn ty() -> Type;
}

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

    fn expr(&self) -> lang::Expr {
        expr_reg::get(self.expr_id())
    }

    fn map_expr(self, f: impl FnOnce(Expr) -> Expr) -> Self {
        Self::from_expr(f(self.expr()))
    }
}

pub trait ScalarType: Clone + ValueType + Into<lang::Lit> {}

impl ValueType for bool {
    fn ty() -> lang::Type {
        lang::Type::U32
    }
}

impl ValueType for u32 {
    fn ty() -> lang::Type {
        lang::Type::U32
    }
}

impl ValueType for f32 {
    fn ty() -> lang::Type {
        lang::Type::F32
    }
}

impl ScalarType for bool {}

impl ScalarType for u32 {}

impl ScalarType for f32 {}

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

impl<T> From<T> for Scalar<T>
where
    T: ScalarType,
{
    fn from(x: T) -> Self {
        Self::from_expr(lang::Expr::Lit(lang::ExprLit { lit: x.into() }))
    }
}

impl<T, Rhs> Add<Rhs> for Scalar<T>
where
    T: ScalarType,
    Rhs: Into<Scalar<T>>,
{
    type Output = Scalar<T>;

    fn add(self, rhs: Rhs) -> Scalar<T> {
        Scalar::from_expr(lang::Expr::Binary(lang::ExprBinary {
            left: Box::new(self.expr().clone()),
            op: lang::BinOp::Add,
            right: Box::new(rhs.into().expr().clone()),
        }))
    }
}

impl<T, Rhs> Mul<Rhs> for Scalar<T>
where
    T: ScalarType,
    Rhs: Into<Scalar<T>>,
{
    type Output = Scalar<T>;

    fn mul(self, rhs: Rhs) -> Scalar<T> {
        Scalar::from_expr(lang::Expr::Binary(lang::ExprBinary {
            left: Box::new(self.expr().clone()),
            op: lang::BinOp::Mul,
            right: Box::new(rhs.into().expr().clone()),
        }))
    }
}

pub fn func_call<V>(
    name: impl Into<String>,
    params: Vec<lang::Var>,
    args: Vec<lang::Expr>,
    result: V,
) -> V
where
    V: Value,
{
    assert!(params.len() == args.len());

    let func = lang::Func::UserDefined {
        name: lang::Ident::new(name),
        params,
        result: Box::new(result.expr().clone()),
    };
    let expr = lang::Expr::Call(lang::ExprCall { func, args });

    V::from_expr(expr)
}

pub fn eval<V>(init: V) -> V
where
    V: Value,
{
    let var = lang::Var {
        ident: lang::Ident::new("var"),
        ty: V::Type::ty(),
    };

    let init = Some(Box::new(init.expr()));

    let expr = lang::Expr::Var(lang::ExprVar { var, init });

    Value::from_expr(expr)
}

pub fn cond<B, V>(cond: B, true_value: V, false_value: V) -> V
where
    B: Into<Scalar<bool>>,
    V: Value,
{
    let cond = Box::new(cond.into().expr());
    let true_expr = Box::new(true_value.expr());
    let false_expr = Box::new(false_value.expr());

    let expr = lang::Expr::Cond(lang::ExprCond {
        cond,
        true_expr,
        false_expr,
    });

    Value::from_expr(expr)
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
