use std::collections::{BTreeMap, BTreeSet};

use super::{SimplifiedExpr, VarForm, VarId};

pub type ScopeId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum NodeId {
    Var(VarId),
    Scope(ScopeId),
}

pub struct Scope {
    pub pred_var_id: VarId,
    pub expr: Box<SimplifiedExpr>,
}

pub struct ScopeForm {}

impl ScopeForm {
    pub fn from_var_form(var_form: VarForm) -> Self {
        let mut scopes: BTreeMap<ScopeId, Scope> = BTreeMap::new();
        let mut preds: BTreeMap<VarId, BTreeSet<NodeId>> = BTreeMap::new();

        let mut add_scope = |scope: Scope, preds: &mut BTreeMap<VarId, BTreeSet<NodeId>>| {
            let scope_id = scopes.len();

            successors(&scope.expr, &mut |succ| {
                preds
                    .entry(succ)
                    .or_default()
                    .insert(NodeId::Scope(scope_id));
            });

            scopes.insert(scope_id, scope);
        };

        for (var_id, var_expr) in var_form.var_exprs().iter().enumerate() {
            use SimplifiedExpr::*;

            match var_expr {
                Branch { yes, no, .. } => {
                    add_scope(
                        Scope {
                            pred_var_id: var_id,
                            expr: yes.clone(),
                        },
                        &mut preds,
                    );
                    add_scope(
                        Scope {
                            pred_var_id: var_id,
                            expr: no.clone(),
                        },
                        &mut preds,
                    );
                }
                Arg { .. }
                | ScalarLiteral { .. }
                | Binary { .. }
                | CallFunc { .. }
                | Field { .. }
                | Var { .. } => (),
            }

            //println!("var {} = {} @ {:?}", var_id, var_expr, deps);

            successors(&var_expr, &mut |succ| {
                preds.entry(succ).or_default().insert(NodeId::Var(var_id));
            });
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
