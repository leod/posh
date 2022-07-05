use crate::lang::{Expr, Ident, VarExpr};

use super::{
    expr_reg::{self, ExprId},
    ValueBase,
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

    pub fn from_ident<R: ValueBase>(ident: Ident) -> Self {
        Self::new(Expr::Var(VarExpr {
            ident,
            ty: <R::Rep as ValueBase>::ty(),
            init: None,
        }))
    }

    pub fn expr(&self) -> Expr {
        expr_reg::get(self.expr_id)
    }
}
