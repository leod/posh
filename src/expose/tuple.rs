use std::rc::Rc;

use crate::lang::{CallExpr, Expr, Func, StructFunc, StructTy, Ty};

use super::{common_field_base, field, Expose, FuncArg, Rep, Representative, Trace, Value};

impl<U, V> Expose for (U, V)
where
    U: Expose,
    V: Expose,
{
    type Rep = (Rep<U>, Rep<V>);
}

impl<U, V> Representative for (U, V)
where
    U: Representative,
    V: Representative,
{
}

impl<U, V> FuncArg for (U, V)
where
    U: Value,
    V: Value,
{
    fn ty() -> Ty {
        Ty::Struct(StructTy {
            name: "Pair".into(),
            fields: vec![("x0".into(), U::ty()), ("x1".into(), V::ty())],
        })
    }

    fn from_var_name(name: &str) -> Self {
        Self::from_trace(Trace::from_var_name::<Self>(name))
    }

    fn expr(&self) -> Rc<Expr> {
        let args = vec![self.0.expr(), self.1.expr()];

        if let Some(common_base) = common_field_base(&Self::ty(), &args) {
            common_base
        } else {
            let ty = match <Self as FuncArg>::ty() {
                Ty::Struct(ty) => ty,
                _ => unreachable!(),
            };
            let func = Func::Struct(StructFunc { ty });
            Rc::new(Expr::Call(CallExpr { func, args }))
        }
    }
}

impl<U, V> Value for (U, V)
where
    U: Value,
    V: Value,
{
    fn from_trace(trace: Trace) -> Self {
        assert!(trace.expr().ty() == <Self::Rep as FuncArg>::ty());

        (field(trace, "p0"), field(trace, "p1"))
    }
}
