use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use super::Expr;

thread_local! {
    static REGISTRY: RefCell<Registry> = RefCell::new(Registry::default());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Trace(usize);

impl Trace {
    pub fn new(expr: Expr) -> Self {
        let id = REGISTRY.with(move |reg| {
            let mut reg = reg.borrow_mut();
            reg.insert(expr)
        });

        Trace(id)
    }

    pub fn expr(&self) -> Rc<Expr> {
        REGISTRY.with(|reg| reg.borrow().get(self.0))
    }
}

#[derive(Default)]
struct Registry {
    next_id: usize,
    exprs: BTreeMap<usize, Rc<Expr>>,
}

impl Registry {
    fn insert(&mut self, expr: Expr) -> usize {
        let expr = Rc::new(expr);

        let id = self.next_id;
        self.next_id += 1;

        self.exprs.insert(id, expr);

        id
    }

    fn get(&self, id: usize) -> Rc<Expr> {
        self.exprs.get(&id).unwrap().clone()
    }
}
