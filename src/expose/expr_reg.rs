use std::{cell::RefCell, collections::BTreeMap, fmt, thread_local, rc::Rc};

use crate::lang::Expr;

// The skeletons are buried in a thread local global variable.

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprId(usize);

impl fmt::Debug for ExprId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", get(*self))
    }
}

struct ExprReg {
    next_id: ExprId,
    exprs: BTreeMap<ExprId, Rc<Expr>>,
    expr_ids: BTreeMap<Rc<Expr>, ExprId>,
}

thread_local! {
    static EXPR_REG: RefCell<ExprReg> = RefCell::new(ExprReg {
        next_id: ExprId(0),
        exprs: BTreeMap::new(),
        expr_ids: BTreeMap::new(),
    });
}

pub fn put(expr: Expr) -> ExprId {
    EXPR_REG.with(move |reg| {
        let mut reg = reg.borrow_mut();

        let id = reg.next_id;
        reg.next_id.0 += 1;

        let expr = Rc::new(expr);

        reg.exprs.insert(id, expr.clone());
        reg.expr_ids.insert(expr, id);

        id
    })
}

pub fn get(id: ExprId) -> Rc<Expr> {
    EXPR_REG.with(|reg| reg.borrow().exprs.get(&id).unwrap().clone())
}
