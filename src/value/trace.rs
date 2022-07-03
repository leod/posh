use crate::lang::{Expr, Ident, VarExpr};

use super::{
    expr_reg::{self, ExprId},
    TypedVal,
};

#[derive(Debug, Copy, Clone)]
pub struct Trace {
    expr_id: ExprId,
}

impl Trace {
    pub fn new(expr: Expr) -> Self {
        Self {
            expr_id: expr_reg::put(expr),
        }
    }

    pub fn from_ident<R: TypedVal>(ident: Ident) -> Self {
        Self::new(Expr::Var(VarExpr {
            ident,
            ty: <R::Val as TypedVal>::ty(),
            init: None,
        }))
    }

    pub fn expr(&self) -> Expr {
        expr_reg::get(self.expr_id)
    }
}
