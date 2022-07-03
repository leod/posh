use super::{ConstructibleVal, Type, TypedVal, Val, Value};

impl<U, V> Type for (U, V)
where
    U: Type,
    V: Type,
{
    type Val = (Value<U>, Value<V>);
}

impl<U, V> Val for (U, V)
where
    U: Val,
    V: Val,
{
}

impl<U, V> TypedVal for (U, V)
where
    U: ConstructibleVal,
    V: ConstructibleVal,
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

impl<U, V> ConstructibleVal for (U, V)
where
    U: ConstructibleVal,
    V: ConstructibleVal,
{
    fn from_trace(trace: super::Trace) -> Self {
        todo!()
    }
}
