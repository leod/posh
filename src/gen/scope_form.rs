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

pub struct ScopeForm {}

impl ScopeForm {
    pub fn from_var_form(var_form: VarForm) -> Self {
        let mut scopes: BTreeMap<ScopeId, Scope> = BTreeMap::new();
        let mut preds: BTreeMap<VarId, BTreeSet<NodeId>> = BTreeMap::new();
        let mut var_inits: BTreeMap<VarId, VarInit> = BTreeMap::new();

        let mut add_scope = |scope: Scope, preds: &mut BTreeMap<VarId, BTreeSet<NodeId>>| {
            let scope_id = scopes.len();

            successors(&scope.expr, &mut |succ| {
                preds
                    .entry(succ)
                    .or_default()
                    .insert(NodeId::Scope(scope_id));
            });

            scopes.insert(scope_id, scope);

            scope_id
        };

        for (var_id, var_expr) in var_form.var_exprs().into_iter().enumerate() {
            use SimplifiedExpr::*;

            //println!("var {} = {} @ {:?}", var_id, var_expr, deps);

            successors(&var_expr, &mut |succ| {
                preds.entry(succ).or_default().insert(NodeId::Var(var_id));
            });

            let var_init = match var_expr {
                Branch { cond, yes, no, .. } => {
                    let yes_scope_id = add_scope(
                        Scope {
                            pred_var_id: var_id,
                            expr: yes.clone(),
                            vars: BTreeMap::new(),
                        },
                        &mut preds,
                    );

                    let no_scope_id = add_scope(
                        Scope {
                            pred_var_id: var_id,
                            expr: no.clone(),
                            vars: BTreeMap::new(),
                        },
                        &mut preds,
                    );

                    VarInit::Branch {
                        cond: *cond,
                        yes_scope_id,
                        no_scope_id,
                    }
                }
                Arg { .. }
                | ScalarLiteral { .. }
                | Binary { .. }
                | CallFunc { .. }
                | Field { .. }
                | Var { .. } => VarInit::Expr(var_expr),
            };

            var_inits.insert(var_id, var_init);
        }

        println!("preds = {preds:#?}");

        for (scope_id, scope) in scopes.iter() {
            println!(
                "scope {} in var {}: {}",
                scope_id, scope.pred_var_id, scope.expr,
            );
        }

        // scope id -> var id
        // var id -> var id

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
