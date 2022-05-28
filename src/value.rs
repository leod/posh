use std::marker::PhantomData;

use crate::lang;

pub trait Type {
    fn ty() -> lang::Type;
}

pub trait Value {
    type Type: Type;

    fn expr(&self) -> lang::Expr;
}

pub trait IntegralType: Type {}

impl Type for u32 {
    fn ty() -> lang::Type {
        lang::Type::U32
    }
}

impl IntegralType for u32 {}

pub struct Scalar<T> {
    _phantom: PhantomData<T>,
}
