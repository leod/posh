mod expr;
mod scope_form;
mod var_form;

pub use scope_form::{Scope, ScopeForm};
pub use var_form::{Var, VarForm, VarId};

use std::rc::Rc;

use crate::dag::Expr;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExprKey(*const Expr);

impl<'a> From<&'a Rc<Expr>> for ExprKey {
    fn from(value: &'a Rc<Expr>) -> Self {
        ExprKey(&**value as *const Expr)
    }
}
