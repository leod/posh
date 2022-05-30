use crate::lang::{BinOp, Expr, ExprBinary, ExprCall, ExprCond, ExprVar, Func, Ident, Var};

use super::{Scalar, Value, ValueType};

pub(crate) fn binary<U, V, R>(left: U, op: BinOp, right: V) -> R
where
    U: Value,
    V: Value,
    R: Value,
{
    let left = Box::new(left.expr());
    let right = Box::new(right.expr());

    let expr = Expr::Binary(ExprBinary {
        left,
        op,
        right,
        ty: R::Type::ty(),
    });

    R::from_expr(expr)
}

pub fn and(left: impl Into<Scalar<bool>>, right: impl Into<Scalar<bool>>) -> Scalar<bool> {
    binary(left.into(), BinOp::Add, right.into())
}

pub fn or(left: impl Into<Scalar<bool>>, right: impl Into<Scalar<bool>>) -> Scalar<bool> {
    binary(left.into(), BinOp::Or, right.into())
}

pub fn func_call<V>(name: impl Into<String>, params: Vec<Var>, result: V, args: Vec<Expr>) -> V
where
    V: Value,
{
    assert!(params.len() == args.len());

    let func = Func::UserDefined {
        name: Ident::new(name),
        params,
        result: Box::new(result.expr()),
    };
    let expr = Expr::Call(ExprCall { func, args });

    V::from_expr(expr)
}

pub fn var<V>(init: V) -> V
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

#[macro_export]
macro_rules! bool {
    { $x:tt } => { $x }
}
