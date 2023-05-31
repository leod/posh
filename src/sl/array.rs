use std::{marker::PhantomData, rc::Rc};

use crate::ToSl;

use super::{
    dag::{ArrayType, Expr, Trace, Type},
    primitives::value_arg,
    Object, Value, ValueNonArray, U32,
};

/// An array value in the shading language.
#[derive(Debug, Copy, Clone)]
pub struct Array<V, const N: usize> {
    trace: Trace,
    _phantom: PhantomData<V>,
}

impl<V: ValueNonArray, const N: usize> Object for Array<V, N> {
    fn ty() -> Type {
        Type::Array(ArrayType {
            ty: Box::new(V::ty()),
            len: N,
        })
    }

    fn expr(&self) -> Rc<Expr> {
        self.trace.expr()
    }

    fn from_arg(name: &str) -> Self {
        value_arg(name)
    }
}

impl<V: ValueNonArray, const N: usize> Value for Array<V, N> {
    fn from_expr(expr: Expr) -> Self {
        assert!(expr.ty() == Self::ty());

        Self {
            trace: Trace::new(expr),
            _phantom: PhantomData,
        }
    }
}

impl<V: ToSl, const N: usize> ToSl for [V; N]
where
    V::Output: ValueNonArray,
{
    type Output = Array<V::Output, N>;

    fn to_sl(self) -> Self::Output {
        let args = self.iter().map(|arg| arg.to_sl().expr()).collect();
        let ty = ArrayType {
            ty: Box::new(V::Output::ty()),
            len: N,
        };

        let expr = Expr::ArrayLiteral { args, ty };

        Array::from_expr(expr)
    }
}

impl<V: ValueNonArray, const N: usize> ToSl for Array<V, N> {
    type Output = Self;

    fn to_sl(self) -> Self {
        self
    }
}

impl<V: ValueNonArray, const N: usize> Array<V, N> {
    pub fn len(&self) -> U32 {
        (N as u32).to_sl()
    }

    pub fn is_empty(&self) -> bool {
        N == 0
    }

    pub fn get(&self, index: impl ToSl<Output = U32>) -> V {
        // FIXME: Prevent out-of-bounds access.
        let base = self.trace.expr();
        let index = index.to_sl().expr();
        let ty = V::ty();

        let expr = Expr::Subscript { base, index, ty };

        V::from_expr(expr)
    }
}

pub fn array<V: ValueNonArray, const N: usize>(args: [V; N]) -> Array<V, N> {
    let args = args.iter().map(|arg| arg.expr()).collect();
    let ty = ArrayType {
        ty: Box::new(V::ty()),
        len: N,
    };

    let expr = Expr::ArrayLiteral { args, ty };

    Array::from_expr(expr)
}
