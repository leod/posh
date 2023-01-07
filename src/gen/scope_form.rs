use std::collections::{BTreeMap, BTreeSet};

use super::{SimplifiedExpr, VarForm, VarId};

pub type ScopeId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum NodeId {
    Var(VarId),
    Scope(ScopeId),
}

#[derive(Debug, Clone)]
pub enum VarInit {
    Expr(SimplifiedExpr),
    Branch {
        cond: SimplifiedExpr,
        yes_scope_id: ScopeId,
        no_scope_id: ScopeId,
    },
}

pub struct Scope {
    pub pred_var_id: VarId,
    pub expr: Box<SimplifiedExpr>,
    pub vars: BTreeMap<VarId, VarInit>,
}

#[derive(Default)]
struct Graph {
    scopes: BTreeMap<ScopeId, Scope>,
    preds: BTreeMap<VarId, BTreeSet<NodeId>>,
    var_inits: BTreeMap<VarId, VarInit>,
}

impl Graph {
    fn add_scope(&mut self, scope: Scope) -> usize {
        let scope_id = self.scopes.len();

        successors(&scope.expr, &mut |succ| {
            self.preds
                .entry(succ)
                .or_default()
                .insert(NodeId::Scope(scope_id));
        });

        self.scopes.insert(scope_id, scope);

        scope_id
    }

    fn new(var_form: &VarForm) -> Self {
        let mut graph = Self::default();

        for (var_id, var_expr) in var_form.var_exprs().iter().enumerate() {
            use SimplifiedExpr::*;

            //println!("var {} = {} @ {:?}", var_id, var_expr, deps);

            successors(&var_expr, &mut |succ| {
                graph
                    .preds
                    .entry(succ)
                    .or_default()
                    .insert(NodeId::Var(var_id));
            });

            let var_init = match var_expr {
                Branch { cond, yes, no, .. } => {
                    let yes_scope_id = graph.add_scope(Scope {
                        pred_var_id: var_id,
                        expr: yes.clone(),
                        vars: BTreeMap::new(),
                    });

                    let no_scope_id = graph.add_scope(Scope {
                        pred_var_id: var_id,
                        expr: no.clone(),
                        vars: BTreeMap::new(),
                    });

                    VarInit::Branch {
                        cond: (**cond).clone(),
                        yes_scope_id,
                        no_scope_id,
                    }
                }
                Arg { .. }
                | ScalarLiteral { .. }
                | Binary { .. }
                | CallFunc { .. }
                | Field { .. }
                | Var { .. } => VarInit::Expr((*var_expr).clone()),
            };

            graph.var_inits.insert(var_id, var_init);
        }

        for (scope_id, scope) in graph.scopes.iter() {
            println!(
                "scope {} in var {}: {}",
                scope_id, scope.pred_var_id, scope.expr,
            );
        }

        todo!()
    }
}

pub struct ScopeForm {}

impl ScopeForm {
    pub fn from_var_form(var_form: VarForm) -> Self {
        let graph = Graph::new(&var_form);

        todo!()
    }
}

fn successors(expr: &SimplifiedExpr, f: &mut impl FnMut(VarId)) {
    use SimplifiedExpr::*;

    match expr {
        Branch { cond, .. } => {
            successors(cond, f);
        }
        Arg { .. } | ScalarLiteral { .. } => (),
        Binary { left, right, .. } => {
            successors(left, f);
            successors(right, f);
        }
        CallFunc { args, .. } => {
            for arg in args {
                successors(arg, f);
            }
        }
        Field { base, .. } => {
            successors(base, f);
        }
        Var { id, .. } => {
            f(*id);
        }
    }
}
