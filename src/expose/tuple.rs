use super::{Expose, Rep, Representant, Transparent, Value};

impl<U, V> Expose for (U, V)
where
    U: Expose,
    V: Expose,
{
    type Rep = (Rep<U>, Rep<V>);
}

impl<U, V> Representant for (U, V)
where
    U: Representant,
    V: Representant,
{
}

impl<U, V> Value for (U, V)
where
    U: Transparent,
    V: Transparent,
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

impl<U, V> Transparent for (U, V)
where
    U: Transparent,
    V: Transparent,
{
    fn from_trace(trace: super::Trace) -> Self {
        todo!()
    }
}
