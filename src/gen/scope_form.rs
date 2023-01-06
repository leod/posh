use std::rc::Rc;

use crate::dag::Expr;

use super::{VarForm, VarId};

pub enum Node {
    Scope(Rc<Expr>),
    Var(VarId),
}

pub struct Scope {}

pub struct ScopeForm {}

impl ScopeForm {
    pub fn from_var_form(var_form: VarForm) -> Self {
        //let mut scope_to_

        todo!()
    }
}
