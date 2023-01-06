use std::collections::{BTreeMap, BTreeSet};

use super::{SimplifiedExpr, VarForm, VarId};

pub type ScopeId = usize;

pub struct Scope {
    pub expr: Box<SimplifiedExpr>,
}

pub struct ScopeForm {}

impl ScopeForm {
    pub fn from_var_form(var_form: VarForm) -> Self {
        let mut scopes = BTreeMap::new();

        let mut add_scope = |scope: Scope| {
            let scope_id = scopes.len();
            scopes.insert(scope_id, scope);
        };

        for (var_id, var_expr) in var_form.var_exprs().iter().enumerate() {
            use SimplifiedExpr::*;

            match var_expr {
                Branch { yes, no, .. } => {
                    add_scope(Scope { expr: yes.clone() });
                    add_scope(Scope { expr: no.clone() });
                }
                Arg { .. }
                | ScalarLiteral { .. }
                | Binary { .. }
                | CallFunc { .. }
                | Field { .. }
                | Var { .. } => (),
            }

            let mut deps = BTreeSet::new();
            collect_deps(&var_expr, &mut deps);

            println!("var {} = {} @ {:?}", var_id, var_expr, deps);
        }

        for (scope_id, scope) in scopes.iter() {
            let mut deps = BTreeSet::new();
            collect_deps(&scope.expr, &mut deps);

            println!("scope {}: {} @ {:?}", scope_id, scope.expr, deps);
        }

        todo!()
    }
}

fn collect_deps(expr: &SimplifiedExpr, deps: &mut BTreeSet<VarId>) {
    use SimplifiedExpr::*;

    match expr {
        Branch { cond, yes, no, .. } => {
            collect_deps(cond, deps);
            collect_deps(yes, deps);
            collect_deps(no, deps);
        }
        Arg { .. } | ScalarLiteral { .. } => (),
        Binary { left, right, .. } => {
            collect_deps(left, deps);
            collect_deps(right, deps);
        }
        CallFunc { args, .. } => {
            for arg in args {
                collect_deps(arg, deps);
            }
        }
        Field { base, .. } => {
            collect_deps(base, deps);
        }
        Var { id, .. } => {
            deps.insert(*id);
        }
    }
}
