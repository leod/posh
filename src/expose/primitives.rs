use std::{collections::BTreeSet, rc::Rc};

use crate::lang::{
    BinaryExpr, BinaryOp, BuiltInFunc, CallExpr, DefFunc, Expr, FieldExpr, Func, Ty,
};

use super::{FuncArg, IntoRep, Trace, Value};

#[doc(hidden)]
pub fn common_field_base(target_ty: &Ty, exprs: &[Rc<Expr>]) -> Option<Rc<Expr>> {
    exprs.first().and_then(|first_expr| {
        if let Expr::Field(first_field_expr) = &**first_expr {
            let mut fields = BTreeSet::new();

            for expr in exprs {
                if let Expr::Field(field_expr) = &**expr {
                    if field_expr.base.ty() != *target_ty
                        || field_expr.base != first_field_expr.base
                    {
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
                    Some(first_field_expr.base.clone())
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
        base: base.expr(),
        member: member.into(),
        ty: <R::Rep as FuncArg>::ty(),
    });

    R::from_expr(expr)
}

#[doc(hidden)]
pub fn func_def_and_call<R: Value>(def: DefFunc, args: Vec<Rc<Expr>>) -> R {
    assert!(def.params.len() == args.len());

    let func = Func::Def(def);
    let expr = Expr::Call(CallExpr { func, args });

    R::from_expr(expr)
}

pub(crate) fn binary<U, V, R>(
    left: impl IntoRep<Rep = U>,
    op: BinaryOp,
    right: impl IntoRep<Rep = V>,
) -> R
where
    U: FuncArg,
    V: FuncArg,
    R: Value,
{
    let left = left.into_rep().expr();
    let right = right.into_rep().expr();

    let expr = Expr::Binary(BinaryExpr {
        left,
        op,
        right,
        ty: <R::Rep as FuncArg>::ty(),
    });

    R::from_expr(expr)
}

pub(crate) fn built_in1<U, R>(name: &str, u: impl IntoRep<Rep = U>) -> R
where
    U: FuncArg,
    R: Value,
{
    let func = Func::BuiltIn(BuiltInFunc {
        name: name.into(),
        ty: <R::Rep as FuncArg>::ty(),
    });
    let expr = Expr::Call(CallExpr {
        func,
        args: vec![u.into_rep().expr()],
    });

    R::from_expr(expr)
}

pub(crate) fn built_in2<U, V, R>(
    name: &str,
    u: impl IntoRep<Rep = U>,
    v: impl IntoRep<Rep = V>,
) -> R
where
    U: FuncArg,
    V: FuncArg,
    R: Value,
{
    let func = Func::BuiltIn(BuiltInFunc {
        name: name.into(),
        ty: <R::Rep as FuncArg>::ty(),
    });
    let expr = Expr::Call(CallExpr {
        func,
        args: vec![u.into_rep().expr(), v.into_rep().expr()],
    });

    R::from_expr(expr)
}

pub(crate) fn built_in3<U, V, W, R>(
    name: &str,
    u: impl IntoRep<Rep = U>,
    v: impl IntoRep<Rep = V>,
    w: impl IntoRep<Rep = W>,
) -> R
where
    U: FuncArg,
    V: FuncArg,
    W: FuncArg,
    R: Value,
{
    let func = Func::BuiltIn(BuiltInFunc {
        name: name.into(),
        ty: <R::Rep as FuncArg>::ty(),
    });
    let expr = Expr::Call(CallExpr {
        func,
        args: vec![
            u.into_rep().expr(),
            v.into_rep().expr(),
            w.into_rep().expr(),
        ],
    });

    R::from_expr(expr)
}

pub(crate) fn built_in4<U, V, W, X, R>(
    name: &str,
    u: impl IntoRep<Rep = U>,
    v: impl IntoRep<Rep = V>,
    w: impl IntoRep<Rep = W>,
    x: impl IntoRep<Rep = X>,
) -> R
where
    U: FuncArg,
    V: FuncArg,
    W: FuncArg,
    X: FuncArg,
    R: Value,
{
    let func = Func::BuiltIn(BuiltInFunc {
        name: name.into(),
        ty: <R::Rep as FuncArg>::ty(),
    });
    let expr = Expr::Call(CallExpr {
        func,
        args: vec![
            u.into_rep().expr(),
            v.into_rep().expr(),
            w.into_rep().expr(),
            x.into_rep().expr(),
        ],
    });

    R::from_expr(expr)
}
