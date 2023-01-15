use std::collections::{BTreeMap, BTreeSet};

use crate::dag::Type;

use super::{
    simplified_expr::{SimplifiedExpr, VarId},
    var_form::VarForm,
};

pub type ScopeId = usize;

#[derive(Debug, Clone)]
pub enum VarInit<'a> {
    Expr(&'a SimplifiedExpr),
    Branch {
        cond: &'a SimplifiedExpr,
        yes_id: ScopeId,
        no_id: ScopeId,
        ty: &'a Type,
    },
}

#[derive(Debug, Clone, Default)]
struct VarState {
    scope_ids: BTreeSet<ScopeId>,
}

#[derive(Debug, Clone)]
pub struct Scope<'a> {
    pub parent_id: Option<ScopeId>,
    pub depth: usize,
    pub vars: BTreeMap<VarId, VarInit<'a>>,
    pub result: Option<&'a SimplifiedExpr>,
}

#[derive(Debug, Clone, Default)]
pub struct ScopeForm<'a> {
    scopes: BTreeMap<ScopeId, Scope<'a>>,
    root_scope_id: ScopeId,
    var_states: BTreeMap<VarId, VarState>,
}

impl<'a> ScopeForm<'a> {
    pub fn new(var_form: &'a VarForm) -> Self {
        let mut scope_form = Self::default();

        scope_form.root_scope_id = scope_form.add(
            Scope {
                parent_id: None,
                depth: 0,
                vars: BTreeMap::new(),
                result: None,
            },
            &var_form.simplified_roots(),
        );

        for (var_id, var_expr) in var_form.var_exprs().rev() {
            let parent_id = scope_form.var_states[&var_id]
                .scope_ids
                .iter()
                .copied()
                .reduce(|acc_id, scope_id| scope_form.lca(acc_id, scope_id))
                .unwrap();

            use SimplifiedExpr::*;

            let var_init = match var_expr {
                Branch { cond, yes, no, ty } => {
                    scope_form.insert_deps(parent_id, cond);

                    let yes_id = scope_form.add(
                        Scope {
                            parent_id: Some(parent_id),
                            depth: scope_form.scopes[&parent_id].depth + 1,
                            vars: BTreeMap::new(),
                            result: Some(yes),
                        },
                        &[yes],
                    );
                    let no_id = scope_form.add(
                        Scope {
                            parent_id: Some(parent_id),
                            depth: scope_form.scopes[&parent_id].depth + 1,
                            vars: BTreeMap::new(),
                            result: Some(no),
                        },
                        &[no],
                    );

                    VarInit::Branch {
                        cond,
                        yes_id,
                        no_id,
                        ty,
                    }
                }
                Arg { .. }
                | ScalarLiteral { .. }
                | Binary { .. }
                | CallFunc { .. }
                | Subscript { .. }
                | Field { .. }
                | Var { .. } => {
                    scope_form.insert_deps(parent_id, var_expr);

                    VarInit::Expr(var_expr)
                }
            };

            scope_form
                .scopes
                .get_mut(&parent_id)
                .unwrap()
                .vars
                .insert(var_id, var_init);
        }

        scope_form
    }

    pub fn scope(&self, scope_id: ScopeId) -> &Scope {
        &self.scopes[&scope_id]
    }

    pub fn root_scope(&self) -> &Scope {
        self.scope(self.root_scope_id)
    }

    fn insert_deps(&mut self, scope_id: ScopeId, expr: &SimplifiedExpr) {
        unscoped_successors(expr, &mut |succ| {
            self.var_states
                .entry(succ)
                .or_default()
                .scope_ids
                .insert(scope_id);
        });
    }

    fn add(&mut self, scope: Scope<'a>, exprs: &[&'a SimplifiedExpr]) -> ScopeId {
        let scope_id = self.scopes.len();

        for expr in exprs {
            self.insert_deps(scope_id, expr);
        }

        self.scopes.insert(scope_id, scope);

        scope_id
    }

    fn lca(&self, mut u_id: ScopeId, mut v_id: ScopeId) -> ScopeId {
        let s = |id: ScopeId| &self.scopes[&id];

        while s(u_id).depth != s(v_id).depth {
            if s(u_id).depth > s(v_id).depth {
                u_id = s(u_id).parent_id.unwrap();
            } else {
                v_id = s(v_id).parent_id.unwrap();
            }
        }

        while u_id != v_id {
            u_id = s(u_id).parent_id.unwrap();
            v_id = s(v_id).parent_id.unwrap();
        }

        u_id
    }
}

fn unscoped_successors(expr: &SimplifiedExpr, f: &mut impl FnMut(VarId)) {
    use SimplifiedExpr::*;

    match expr {
        Branch { cond, .. } => {
            unscoped_successors(cond, f);
        }
        Arg { .. } | ScalarLiteral { .. } => (),
        Binary { left, right, .. } => {
            unscoped_successors(left, f);
            unscoped_successors(right, f);
        }
        CallFunc { args, .. } => {
            for arg in args {
                unscoped_successors(arg, f);
            }
        }
        Field { base, .. } => {
            unscoped_successors(base, f);
        }
        Subscript { base, index, .. } => {
            unscoped_successors(base, f);
        }
        Var { id, .. } => {
            f(*id);
        }
    }
}
