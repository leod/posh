use std::rc::Rc;

use super::{
    dag::{BinaryOp, Expr, FuncDef, StructType, Type, UnaryOp},
    Bool, Object, ToSl, Value,
};

pub fn any(vs: impl IntoIterator<Item = Bool>) -> Bool {
    vs.into_iter().fold(false.to_sl(), |x, v| x.or(v))
}

pub(crate) fn cast<U, V>(u: impl ToSl<Output = U>) -> V
where
    U: Value,
    V: Value,
{
    built_in_1(&format!("{}", V::ty()), u.to_sl())
}

pub(crate) fn unary<U, R>(op: UnaryOp, arg: impl ToSl<Output = U>) -> R
where
    U: Value,
    R: Value,
{
    let ty = R::ty();
    let arg = arg.to_sl().expr();

    let expr = Expr::Unary { ty, op, arg };

    R::from_expr(expr)
}

pub(crate) fn binary<U, V, R>(
    left: impl ToSl<Output = U>,
    op: BinaryOp,
    right: impl ToSl<Output = V>,
) -> R
where
    U: Value,
    V: Value,
    R: Value,
{
    let ty = R::ty();
    let left = left.to_sl().expr();
    let right = right.to_sl().expr();

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
    u: impl ToSl<Output = U>,
    v: impl ToSl<Output = V>,
    w: impl ToSl<Output = W>,
    x: impl ToSl<Output = X>,
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
        u.to_sl().expr(),
        v.to_sl().expr(),
        w.to_sl().expr(),
        x.to_sl().expr(),
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
