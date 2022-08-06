use crate::lang::{Expr, Func, FuncDef, Ident};

use super::{FuncArg, Value};

pub fn compile1<U, R>(f: fn(U) -> R) -> Option<FuncDef>
where
    U: FuncArg,
    R: Value,
{
    let u = U::from_ident(Ident::new("u"));
    let r = f(u);

    if let Expr::Call(expr) = &*r.expr() {
        if let Func::Def(ref func) = expr.func {
            Some(func.clone())
        } else {
            None
        }
    } else {
        None
    }
}