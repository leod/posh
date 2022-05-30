use crate::{
    lang::{BinOp, Expr, ExprBinary, ExprCall, ExprTernary, ExprVar, Func, Ident, Var},
    Bool,
};

use super::{IntoValue, Scalar, Value, ValueType};

pub(crate) fn binary<U, V, R>(
    left: impl IntoValue<Value = U>,
    op: BinOp,
    right: impl IntoValue<Value = V>,
) -> R
where
    U: Value,
    V: Value,
    R: Value,
{
    let left = Box::new(left.into_value().expr());
    let right = Box::new(right.into_value().expr());

    let expr = Expr::Binary(ExprBinary {
        left,
        op,
        right,
        ty: R::Type::ty(),
    });

    R::from_expr(expr)
}

pub fn and(left: impl IntoValue<Value = Bool>, right: impl IntoValue<Value = Bool>) -> Bool {
    binary(left, BinOp::Add, right)
}

pub fn or(left: impl IntoValue<Value = Bool>, right: impl IntoValue<Value = Bool>) -> Bool {
    binary(left, BinOp::Add, right)
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

pub fn ternary<B, V>(cond: B, true_value: impl Into<V>, false_value: impl Into<V>) -> V
where
    B: Into<Scalar<bool>>,
    V: Value,
{
    let cond = Box::new(cond.into().expr());
    let true_expr = Box::new(true_value.into().expr());
    let false_expr = Box::new(false_value.into().expr());

    let expr = Expr::Ternary(ExprTernary {
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
