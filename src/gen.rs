pub mod expr;
pub mod scope;
pub mod topo;

use std::rc::Rc;

use crate::dag::Expr;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExprKey(*const Expr);

impl<'a> From<&'a Rc<Expr>> for ExprKey {
    fn from(value: &'a Rc<Expr>) -> Self {
        ExprKey(&**value as *const Expr)
    }
}