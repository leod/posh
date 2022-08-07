use std::{collections::BTreeMap, rc::Rc};

use super::Expr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ExprId(usize);

/*impl fmt::Debug for ExprId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", get(*self))
    }
}*/

#[derive(Default)]
pub struct ExprReg {
    next_id: ExprId,
    exprs: BTreeMap<ExprId, Rc<Expr>>,
}

impl ExprReg {
    pub fn new() -> Self {
        ExprReg::default()
    }

    pub fn insert(&mut self, expr: Expr) -> ExprId {
        let expr = Rc::new(expr);

        let id = self.next_id;
        self.next_id.0 += 1;

        self.exprs.insert(id, expr);

        id
    }

    pub fn get(&self, id: ExprId) -> Rc<Expr> {
        self.exprs.get(&id).unwrap().clone()
    }
}
