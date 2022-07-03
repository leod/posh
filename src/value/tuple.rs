use crate::{Po, ValueBase};

use super::{Lift, Value};

impl<U, V> Lift for (U, V)
where
    U: Lift,
    V: Lift,
{
    type Type = (Po<U>, Po<V>);
}

impl<U, V> ValueBase for (U, V)
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
