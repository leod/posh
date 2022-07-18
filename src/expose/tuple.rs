use super::{Expose, FuncArg, Rep, Representative, Value};

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
    fn ty() -> crate::lang::Ty {
        todo!()
    }

    fn from_ident(ident: crate::lang::Ident) -> Self {
        todo!()
    }

    fn expr(&self) -> crate::lang::Expr {
        todo!()
    }
}

impl<U, V> Value for (U, V)
where
    U: Value,
    V: Value,
{
    fn from_trace(trace: super::Trace) -> Self {
        todo!()
    }
}
