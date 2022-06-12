use crate::{Posh, Type, Value};

use super::{Lift, TransparentValue};

impl<U, V> Type for (U, V)
where
    U: Type,
    V: Type,
{
    fn ty() -> crate::lang::Ty {
        todo!()
    }
}

impl<U, V> Lift for (U, V)
where
    U: Lift,
    V: Lift,
{
    type Posh = (Posh<U>, Posh<V>);
}

impl<U, V> Value for (U, V)
where
    U: TransparentValue,
    V: TransparentValue,
{
    type Type = (U::Type, V::Type);

    fn from_ident(ident: crate::lang::Ident) -> Self {
        todo!()
    }

    fn expr(&self) -> crate::lang::Expr {
        todo!()
    }
}

impl<U, V> TransparentValue for (U, V)
where
    U: TransparentValue,
    V: TransparentValue,
{
    fn from_trace(trace: super::Trace) -> Self {
        todo!()
    }
}
