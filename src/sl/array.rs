use std::{marker::PhantomData, rc::Rc};

use super::{
    dag::{ArrayType, Expr, Trace, Type},
    primitives::value_arg,
    Object, ToValue, Value, ValueNonArray, U32,
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

impl<V: ValueNonArray, const N: usize> ToValue for [V; N] {
    type Output = Array<V, N>;

    fn to_value(self) -> Self::Output {
        todo!()
    }
}

impl<V: ValueNonArray, const N: usize> ToValue for Array<V, N> {
    type Output = Self;

    fn to_value(self) -> Self::Output {
        self
    }
}

impl<V: ValueNonArray, const N: usize> Array<V, N> {
    pub fn index(&self, index: impl ToValue<Output = U32>) -> V {
        let base = self.trace.expr();
        let index = index.to_value().expr();
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
