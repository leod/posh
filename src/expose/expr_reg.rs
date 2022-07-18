use std::{cell::RefCell, collections::BTreeMap, fmt, thread_local};

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
    exprs: BTreeMap<ExprId, Expr>,
}

thread_local! {
    static EXPR_REG: RefCell<ExprReg> = RefCell::new(ExprReg {
        next_id: ExprId(0),
        exprs: BTreeMap::new(),
    });
}

pub fn put(expr: Expr) -> ExprId {
    EXPR_REG.with(move |reg| {
        let mut reg = reg.borrow_mut();

        let id = reg.next_id;
        reg.next_id.0 += 1;

        reg.exprs.insert(id, expr);

        id
    })
}

pub fn get(id: ExprId) -> Expr {
    EXPR_REG.with(|reg| reg.borrow().exprs.get(&id).unwrap().clone())
}
