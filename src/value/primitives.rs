use crate::{
    lang::{
        BinaryExpr, BinaryOp, BuiltInFunc, CallExpr, Expr, FieldExpr, Func, Ident, TernaryExpr,
        UserDefinedFunc, VarExpr,
    },
    Bool,
};

use super::{IntoValue, Trace, Type, Value};

pub(crate) fn binary<U, V, R>(
    left: impl IntoValue<Value = U>,
    op: BinaryOp,
    right: impl IntoValue<Value = V>,
) -> R
where
    U: Value,
    V: Value,
    R: Value,
{
    let left = Box::new(left.into_value().expr());
    let right = Box::new(right.into_value().expr());

    let expr = Expr::Binary(BinaryExpr {
        left,
        op,
        right,
        ty: <R::Type as Type>::ty(),
    });

    R::from_expr(expr)
}

pub fn field<R>(base: Trace, member: &str) -> R
where
    R: Value,
{
    let expr = Expr::Field(FieldExpr {
        base: Box::new(base.expr()),
        member: member.into(),
        ty: <R::Type as Type>::ty(),
    });

    R::from_expr(expr)
}

pub(crate) fn builtin1<U, R>(name: &str, x: impl IntoValue<Value = U>) -> R
where
    U: Value,
    R: Value,
{
    let func = Func::BuiltIn(BuiltInFunc {
        name: name.into(),
        ty: <R::Type as Type>::ty(),
    });
    let expr = Expr::Call(CallExpr {
        func,
        args: vec![x.into_value().expr()],
    });

    R::from_expr(expr)
}

pub(crate) fn builtin2<U, V, R>(
    name: &str,
    x: impl IntoValue<Value = U>,
    y: impl IntoValue<Value = V>,
) -> R
where
    U: Value,
    V: Value,
    R: Value,
{
    let func = Func::BuiltIn(BuiltInFunc {
        name: name.into(),
        ty: <R::Type as Type>::ty(),
    });
    let expr = Expr::Call(CallExpr {
        func,
        args: vec![x.into_value().expr(), y.into_value().expr()],
    });

    R::from_expr(expr)
}

pub(crate) fn builtin3<U, V, W, R>(
    name: &str,
    x: impl IntoValue<Value = U>,
    y: impl IntoValue<Value = V>,
    z: impl IntoValue<Value = W>,
) -> R
where
    U: Value,
    V: Value,
    W: Value,
    R: Value,
{
    let func = Func::BuiltIn(BuiltInFunc {
        name: name.into(),
        ty: <R::Type as Type>::ty(),
    });
    let expr = Expr::Call(CallExpr {
        func,
        args: vec![
            x.into_value().expr(),
            y.into_value().expr(),
            z.into_value().expr(),
        ],
    });

    R::from_expr(expr)
}

pub fn and(left: impl IntoValue<Value = Bool>, right: impl IntoValue<Value = Bool>) -> Bool {
    binary(left, BinaryOp::And, right)
}

pub fn or(left: impl IntoValue<Value = Bool>, right: impl IntoValue<Value = Bool>) -> Bool {
    binary(left, BinaryOp::Or, right)
}

pub fn func_call<V>(name: impl Into<String>, params: Vec<VarExpr>, result: V, args: Vec<Expr>) -> V
where
    V: Value,
{
    assert!(params.len() == args.len());

    let func = Func::UserDefined(UserDefinedFunc {
        ident: Ident::new(name),
        params,
        result: Box::new(result.expr()),
    });
    let expr = Expr::Call(CallExpr { func, args });

    V::from_expr(expr)
}

pub fn var<V>(init: V) -> V
where
    V: Value,
{
    let init = Some(Box::new(init.expr()));

    let var = VarExpr {
        ident: Ident::new("var"),
        ty: V::Type::ty(),
        init,
    };

    let expr = Expr::Var(var);

    Value::from_expr(expr)
}

pub fn ternary<V>(
    cond: impl IntoValue<Value = Bool>,
    true_value: impl IntoValue<Value = V>,
    false_value: impl IntoValue<Value = V>,
) -> V
where
    V: Value,
{
    let cond = Box::new(cond.into_value().expr());
    let true_expr = Box::new(true_value.into_value().expr());
    let false_expr = Box::new(false_value.into_value().expr());

    let expr = Expr::Ternary(TernaryExpr {
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
