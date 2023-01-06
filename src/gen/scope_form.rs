use std::{collections::HashMap, rc::Rc};

use crate::{dag::Expr, gen::ExprKey};

use super::{SimplifiedExpr, VarForm, VarId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScopeId(usize);

pub struct Scope {
    pub simplified_expr: SimplifiedExpr,
}

pub struct ScopeForm {}

impl ScopeForm {
    pub fn from_var_form(var_form: VarForm) -> Self {
        let mut scopes = HashMap::new();

        let mut add_scope = |scope: Scope| {
            let scope_id = ScopeId(scopes.len());

            scopes.insert(scope_id, scope);
        };

        /*

            for var in var_form.vars() {
                use Expr::*;

                match &var.simplified_expr.expr {
                    Branch { cond, yes, no, .. } => {
                        println!("{yes}");
                        add_scope(Scope {
                            simplified_expr: var_form.simplified_expr(ExprKey::from(yes)).clone(),
                        });
                        add_scope(Scope {
                            simplified_expr: var_form.simplified_expr(ExprKey::from(no)).clone(),
                        });
                    }
                    Arg { .. }
                    | ScalarLiteral { .. }
                    | StructLiteral { .. }
                    | Binary { .. }
                    | CallFuncDef { .. }
                    | CallBuiltIn { .. }
                    | Field { .. } => (),
                }
            }

            for (scope_id, scope) in scopes.iter() {
                println!(
                    "{}: {} @ {:?}",
                    scope_id.0, scope.simplified_expr.expr, scope.simplified_expr.deps
                );
            }
        */

        todo!()
    }
}
