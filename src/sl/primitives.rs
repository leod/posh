use std::{collections::BTreeSet, rc::Rc};

use crate::dag::{BaseTy, BinaryOp, Expr, FuncDef, StructTy, Ty};

use super::{Object, ToValue, Value};

pub(crate) fn binary<U, V, R>(
    left: impl ToValue<Value = U>,
    op: BinaryOp,
    right: impl ToValue<Value = V>,
) -> R
where
    U: Value,
    V: Value,
    R: Value,
{
    let ty = R::TY;
    let left = left.to_value().expr();
    let right = right.to_value().expr();

    let expr = Expr::Binary {
        ty,
        left,
        op,
        right,
    };

    R::from_expr(expr)
}

pub(crate) fn built_in_1<U, R>(name: &'static str, u: impl ToValue<Value = U>) -> R
where
    U: Object,
    R: Value,
{
    let ty = R::TY;
    let args = vec![u.to_value().expr()];

    let expr = Expr::CallBuiltIn { ty, name, args };

    R::from_expr(expr)
}

pub(crate) fn built_in_2<U, V, R>(
    name: &'static str,
    u: impl ToValue<Value = U>,
    v: impl ToValue<Value = V>,
) -> R
where
    U: Object,
    V: Object,
    R: Value,
{
    let ty = R::TY;
    let args = vec![u.to_value().expr(), v.to_value().expr()];

    let expr = Expr::CallBuiltIn { ty, name, args };

    R::from_expr(expr)
}

pub(crate) fn built_in_3<U, V, W, R>(
    name: &'static str,
    u: impl ToValue<Value = U>,
    v: impl ToValue<Value = V>,
    w: impl ToValue<Value = W>,
) -> R
where
    U: Object,
    V: Object,
    W: Object,
    R: Value,
{
    let ty = R::TY;
    let args = vec![
        u.to_value().expr(),
        v.to_value().expr(),
        w.to_value().expr(),
    ];

    let expr = Expr::CallBuiltIn { ty, name, args };

    R::from_expr(expr)
}

pub(crate) fn built_in_4<U, V, W, X, R>(
    name: &'static str,
    u: impl ToValue<Value = U>,
    v: impl ToValue<Value = V>,
    w: impl ToValue<Value = W>,
    x: impl ToValue<Value = X>,
) -> R
where
    U: Object,
    V: Object,
    W: Object,
    X: Object,
    R: Value,
{
    let ty = R::TY;
    let args = vec![
        u.to_value().expr(),
        v.to_value().expr(),
        w.to_value().expr(),
        x.to_value().expr(),
    ];

    let expr = Expr::CallBuiltIn { ty, name, args };

    R::from_expr(expr)
}

#[doc(hidden)]
pub fn field<R: Value>(base: Rc<Expr>, name: &'static str) -> R {
    let ty = R::TY;

    let expr = Expr::Field { ty, base, name };

    R::from_expr(expr)
}

#[doc(hidden)]
pub fn call_func_def<R: Value>(def: FuncDef, args: Vec<Rc<Expr>>) -> R {
    assert!(def.params.len() == args.len());

    let expr = Expr::CallFuncDef { def, args };

    R::from_expr(expr)
}

#[doc(hidden)]
pub fn simplify_struct_literal(ty: &'static StructTy, args: &[Rc<Expr>]) -> Rc<Expr> {
    assert!(ty.fields.len() == args.len());

    let common_base = common_field_base(ty, args);

    if let Some(common_base) = common_base {
        common_base
    } else {
        let expr = Expr::StructLiteral {
            ty,
            args: args.into(),
        };

        Rc::new(expr)
    }
}

fn common_field_base(struct_ty: &'static StructTy, args: &[Rc<Expr>]) -> Option<Rc<Expr>> {
    let ty = Ty::Base(BaseTy::Struct(struct_ty));

    let first_expr = args.first()?;
    let first_base = if let Expr::Field { base, .. } = &**first_expr {
        Some(base)
    } else {
        None
    }?;

    let is_match = |base: &Rc<Expr>| base.ty() == ty && base == first_base;
    let given_fields: BTreeSet<_> = args
        .iter()
        .map(|arg| match &**arg {
            Expr::Field { base, name, .. } if is_match(base) => Some(*name),
            _ => None,
        })
        .collect::<Option<_>>()?;

    let needed_fields: BTreeSet<_> = struct_ty.fields.iter().map(|(name, _)| *name).collect();

    if given_fields == needed_fields {
        Some(first_base.clone())
    } else {
        None
    }
}
