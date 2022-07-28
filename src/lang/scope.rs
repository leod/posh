use std::rc::Rc;

use super::{expr_reg::{ExprReg, ExprId}, Expr, LiteralExpr};

pub enum ScopeExpr {
    Binary(BinaryScopeExpr),
    Call(CallScopeExpr),
    Literal(LiteralExpr),
    Field(FieldScopeExpr),
}

struct IfExpr {

}

pub struct Scope {
    id: ExprId,
    expr: Expr,
    next: Option<Box<Scope>>,
}

pub struct ScopeBuilder {
    expr_reg: ExprReg,
}

impl ScopeBuilder {
    pub fn run(expr: Rc<Expr>) {
    }
}
