use std::{cell::RefCell, rc::Rc};

use crate::lang::{
    expr_reg::{ExprId, ExprReg},
    Expr, Ident, VarExpr,
};

use super::FuncArg;

#[derive(Debug, Copy, Clone)]
pub struct Trace {
    expr_id: ExprId,
}

impl Trace {
    pub fn new(expr: Expr) -> Self {
        Self {
            expr_id: insert(expr),
        }
    }

    pub fn from_ident<R: FuncArg>(ident: Ident) -> Self {
        Self::new(Expr::Var(VarExpr {
            ident,
            ty: <R::Rep as FuncArg>::ty(),
        }))
    }

    pub fn expr(&self) -> Rc<Expr> {
        get(self.expr_id)
    }
}

// The skeletons are buried in a thread local global variable.
thread_local! {
    static EXPR_REG: RefCell<ExprReg> = RefCell::new(ExprReg::new());
}

fn insert(expr: Expr) -> ExprId {
    EXPR_REG.with(move |reg| {
        let mut reg = reg.borrow_mut();
        reg.insert(expr)
    })
}

fn get(id: ExprId) -> Rc<Expr> {
    EXPR_REG.with(|reg| reg.borrow().get(id))
}
