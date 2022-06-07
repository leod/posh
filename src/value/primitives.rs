use std::{collections::BTreeSet, rc::Rc};

use crate::{
    lang::{
        BinaryExpr, BinaryOp, BuiltInFunc, CallExpr, Expr, FieldExpr, Func, Ident, TernaryExpr, Ty,
        UserDefinedFunc, VarExpr,
    },
    Bool,
};

use super::{IntoValue, Trace, Transparent, Type, Value};

#[doc(hidden)]
pub fn common_field_base(exprs: &[Expr]) -> Option<Expr> {
    exprs.first().and_then(|first_expr| {
        if let Expr::Field(first_field_expr) = first_expr {
            let mut fields = BTreeSet::new();

            for expr in exprs {
                if let Expr::Field(field_expr) = expr {
                    if field_expr.base != first_field_expr.base {
                        return None;
                    }

                    fields.insert(field_expr.member.clone());
                } else {
                    return None;
                }
            }

            if let Ty::Struct(ty) = first_field_expr.base.ty() {
                let needed_fields: BTreeSet<_> =
                    ty.fields.iter().map(|(name, _)| name.clone()).collect();

                if needed_fields == fields {
                    Some((*first_field_expr.base).clone())
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    })
}

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
    let left = Rc::new(left.into_value().expr());
    let right = Rc::new(right.into_value().expr());

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
        base: Rc::new(base.expr()),
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
        result: Rc::new(result.expr()),
    });
    let expr = Expr::Call(CallExpr { func, args });

    V::from_expr(expr)
}

pub fn var<V>(init: V) -> V
where
    V: Value,
    V::Type: Transparent,
{
    let init = Some(Rc::new(init.expr()));

    let var = VarExpr {
        ident: Ident::new("var"),
        ty: <V::Type as Type>::ty(),
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
    V::Type: Transparent,
{
    let cond = Rc::new(cond.into_value().expr());
    let true_expr = Rc::new(true_value.into_value().expr());
    let false_expr = Rc::new(false_value.into_value().expr());

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
                init: Some(Rc::new(expr)),
            },
        ));
    }
}

#[macro_export]
macro_rules! bool {
    { $x:tt } => { $x }
}
