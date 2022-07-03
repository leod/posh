use std::{collections::BTreeSet, rc::Rc};

use crate::lang::{
    BinaryExpr, BinaryOp, BuiltInFunc, CallExpr, Expr, FieldExpr, Func, Ident, Ty, UserDefinedFunc,
    VarExpr,
};

use super::{IntoPosh, Trace, Value, ValueBase};

pub fn var<R: Value>(init: R) -> R {
    let init = Some(Rc::new(init.expr()));

    let var = VarExpr {
        ident: Ident::new("var"),
        ty: <R::Type as ValueBase>::ty(),
        init,
    };

    let expr = Expr::Var(var);

    R::from_expr(expr)
}

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

#[doc(hidden)]
pub fn field<R: Value>(base: Trace, member: &str) -> R {
    let expr = Expr::Field(FieldExpr {
        base: Rc::new(base.expr()),
        member: member.into(),
        ty: <R::Type as ValueBase>::ty(),
    });

    R::from_expr(expr)
}

#[doc(hidden)]
pub fn func_def_and_call<R: Value>(
    name: impl Into<String>,
    params: Vec<VarExpr>,
    result: R,
    args: Vec<Expr>,
) -> R {
    assert!(params.len() == args.len());

    let func = Func::UserDefined(UserDefinedFunc {
        ident: Ident::new(name),
        params,
        result: Rc::new(result.expr()),
    });
    let expr = Expr::Call(CallExpr { func, args });

    R::from_expr(expr)
}

pub(crate) fn binary<U, V, R>(
    left: impl IntoPosh<Type = U>,
    op: BinaryOp,
    right: impl IntoPosh<Type = V>,
) -> R
where
    U: ValueBase,
    V: ValueBase,
    R: Value,
{
    let left = Rc::new(left.into_posh().expr());
    let right = Rc::new(right.into_posh().expr());

    let expr = Expr::Binary(BinaryExpr {
        left,
        op,
        right,
        ty: <R::Type as ValueBase>::ty(),
    });

    R::from_expr(expr)
}

pub(crate) fn builtin1<U, R>(name: &str, u: impl IntoPosh<Type = U>) -> R
where
    U: ValueBase,
    R: Value,
{
    let func = Func::BuiltIn(BuiltInFunc {
        name: name.into(),
        ty: <R::Type as ValueBase>::ty(),
    });
    let expr = Expr::Call(CallExpr {
        func,
        args: vec![u.into_posh().expr()],
    });

    R::from_expr(expr)
}

pub(crate) fn builtin2<U, V, R>(
    name: &str,
    u: impl IntoPosh<Type = U>,
    v: impl IntoPosh<Type = V>,
) -> R
where
    U: ValueBase,
    V: ValueBase,
    R: Value,
{
    let func = Func::BuiltIn(BuiltInFunc {
        name: name.into(),
        ty: <R::Type as ValueBase>::ty(),
    });
    let expr = Expr::Call(CallExpr {
        func,
        args: vec![u.into_posh().expr(), v.into_posh().expr()],
    });

    R::from_expr(expr)
}

pub(crate) fn builtin3<U, V, W, R>(
    name: &str,
    u: impl IntoPosh<Type = U>,
    v: impl IntoPosh<Type = V>,
    w: impl IntoPosh<Type = W>,
) -> R
where
    U: ValueBase,
    V: ValueBase,
    W: ValueBase,
    R: Value,
{
    let func = Func::BuiltIn(BuiltInFunc {
        name: name.into(),
        ty: <R::Type as ValueBase>::ty(),
    });
    let expr = Expr::Call(CallExpr {
        func,
        args: vec![
            u.into_posh().expr(),
            v.into_posh().expr(),
            w.into_posh().expr(),
        ],
    });

    R::from_expr(expr)
}

pub(crate) fn builtin4<U, V, W, X, R>(
    name: &str,
    u: impl IntoPosh<Type = U>,
    v: impl IntoPosh<Type = V>,
    w: impl IntoPosh<Type = W>,
    x: impl IntoPosh<Type = X>,
) -> R
where
    U: ValueBase,
    V: ValueBase,
    W: ValueBase,
    X: ValueBase,
    R: Value,
{
    let func = Func::BuiltIn(BuiltInFunc {
        name: name.into(),
        ty: <R::Type as ValueBase>::ty(),
    });
    let expr = Expr::Call(CallExpr {
        func,
        args: vec![
            u.into_posh().expr(),
            v.into_posh().expr(),
            w.into_posh().expr(),
            x.into_posh().expr(),
        ],
    });

    R::from_expr(expr)
}
