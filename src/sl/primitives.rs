use std::rc::Rc;

use super::{
    dag::{BinaryOp, Expr, FuncDef, StructType, Type},
    Object, ToValue, Value,
};

pub(crate) fn cast<U, V>(u: impl ToValue<Output = U>) -> V
where
    U: Value,
    V: Value,
{
    built_in_1(&format!("{}", V::ty()), u.to_value())
}

pub(crate) fn binary<U, V, R>(
    left: impl ToValue<Output = U>,
    op: BinaryOp,
    right: impl ToValue<Output = V>,
) -> R
where
    U: Value,
    V: Value,
    R: Value,
{
    let ty = R::ty();
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

pub(crate) fn built_in_1<U, R>(name: &str, u: U) -> R
where
    U: Object,
    R: Value,
{
    let ty = R::ty();
    let name = name.to_string();
    let args = vec![u.expr()];

    let expr = Expr::CallBuiltIn { ty, name, args };

    R::from_expr(expr)
}

pub(crate) fn built_in_2<U, V, R>(name: &str, u: U, v: V) -> R
where
    U: Object,
    V: Object,
    R: Value,
{
    let ty = R::ty();
    let name = name.to_string();
    let args = vec![u.expr(), v.expr()];

    let expr = Expr::CallBuiltIn { ty, name, args };

    R::from_expr(expr)
}

pub(crate) fn built_in_3<U, V, W, R>(name: &str, u: U, v: V, w: W) -> R
where
    U: Object,
    V: Object,
    W: Object,
    R: Value,
{
    let ty = R::ty();
    let name = name.to_string();
    let args = vec![u.expr(), v.expr(), w.expr()];

    let expr = Expr::CallBuiltIn { ty, name, args };

    R::from_expr(expr)
}

pub(crate) fn built_in_4<U, V, W, X, R>(
    name: &str,
    u: impl ToValue<Output = U>,
    v: impl ToValue<Output = V>,
    w: impl ToValue<Output = W>,
    x: impl ToValue<Output = X>,
) -> R
where
    U: Object,
    V: Object,
    W: Object,
    X: Object,
    R: Value,
{
    let ty = R::ty();
    let name = name.to_string();
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
    let ty = R::ty();

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
pub fn simplify_struct_literal(ty: Rc<StructType>, args: &[Rc<Expr>]) -> Rc<Expr> {
    assert!(ty.fields.len() == args.len());

    let fields = ty.fields.iter().map(|(name, _)| name.as_str());
    let common_base = common_field_base(&Type::Struct(ty.clone()), fields, args);

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

#[doc(hidden)]
pub fn value_arg<R: Value>(name: &str) -> R {
    R::from_expr(Expr::Arg {
        ty: R::ty(),
        name: name.into(),
    })
}

#[doc(hidden)]
pub fn common_field_base<'a>(
    ty: &Type,
    required_fields: impl Iterator<Item = &'a str>,
    args: &[Rc<Expr>],
) -> Option<Rc<Expr>> {
    let required_fields: Vec<_> = required_fields.collect();

    if required_fields.len() != args.len() {
        return None;
    }

    let first_expr = args.first()?;
    let first_base = if let Expr::Field { base, .. } = &**first_expr {
        base
    } else {
        return None;
    };

    if first_base.ty() != *ty {
        return None;
    }

    for (required_field, arg) in required_fields.into_iter().zip(args) {
        if let Expr::Field { base, name, .. } = &**arg {
            if !Rc::ptr_eq(base, first_base) || *name != required_field {
                return None;
            }
        } else {
            return None;
        }
    }

    Some(first_base.clone())
}
