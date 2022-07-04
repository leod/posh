use super::{Constructible, Lift, Val, Value, ValueBase};

impl<U, V> Lift for (U, V)
where
    U: Lift,
    V: Lift,
{
    type Value = (Val<U>, Val<V>);
}

impl<U, V> ValueBase for (U, V)
where
    U: ValueBase,
    V: ValueBase,
{
}

impl<U, V> Value for (U, V)
where
    U: Constructible,
    V: Constructible,
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

impl<U, V> Constructible for (U, V)
where
    U: Constructible,
    V: Constructible,
{
    fn from_trace(trace: super::Trace) -> Self {
        todo!()
    }
}
