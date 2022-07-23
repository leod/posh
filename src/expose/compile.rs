use crate::lang::{DefFunc, Expr, Func, Ident};

use super::{FuncArg, Value};

pub fn compile1<U, R>(f: fn(U) -> R) -> Option<DefFunc>
where
    U: FuncArg,
    R: Value,
{
    let u = U::from_ident(Ident::new("u"));
    let r = f(u);

    if let Expr::Call(expr) = r.expr() {
        if let Func::Def(func) = expr.func {
            Some(func)
        } else {
            None
        }
    } else {
        None
    }
}
