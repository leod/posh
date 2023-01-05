use std::{collections::HashMap, rc::Rc};

use crate::dag::Expr;

use super::{
    topo::{count_usages, topological_ordering},
    ExprKey,
};

#[derive(Debug, Clone)]
pub struct Var {
    name: String,
    expr: Expr,
}

#[derive(Default)]
pub struct Scope {
    vars: HashMap<ExprKey, Var>,
}

fn var_name(idx: usize) -> String {
    format!("var_{idx}")
}

impl Scope {
    pub fn map_expr(&self, expr: Expr, simplified_exprs: &HashMap<ExprKey, Expr>) -> Expr {
        use Expr::*;

        let lookup = |node: Rc<Expr>| {
            let key = ExprKey::from(&node);
            self.vars.get(&key).map_or_else(
                || {
                    simplified_exprs
                        .get(&key)
                        .cloned()
                        .map_or(node, |simplified_expr| Rc::new(simplified_expr))
                },
                |var| {
                    Rc::new(Arg {
                        ty: var.expr.ty(),
                        name: var.name.clone(),
                    })
                },
            )
        };

        match expr {
            expr @ Arg { .. } | expr @ ScalarLiteral { .. } => expr,
            StructLiteral { args, ty } => StructLiteral {
                args: args.into_iter().map(lookup).collect(),
                ty,
            },
            Binary {
                left,
                op,
                right,
                ty,
            } => Binary {
                left: lookup(left),
                op,
                right: lookup(right),
                ty,
            },
            CallFuncDef { .. } => todo!(),
            CallBuiltIn { name, args, ty } => CallBuiltIn {
                name,
                args: args.into_iter().map(lookup).collect(),
                ty,
            },
            Field { base, name, ty } => Field {
                base: lookup(base),
                name,
                ty,
            },
            Branch { cond, yes, no, ty } => Branch {
                cond: lookup(cond),
                yes: lookup(yes),
                no: lookup(no),
                ty,
            },
        }
    }

    fn needs_var(count: usize, expr: &Expr) -> bool {
        use Expr::*;

        match expr {
            Branch { .. } => true,
            Arg { .. } | ScalarLiteral { .. } => false,
            StructLiteral { .. }
            | Binary { .. }
            | CallFuncDef { .. }
            | CallBuiltIn { .. }
            | Field { .. } => count > 1,
        }
    }

    pub fn boop(roots: &[Rc<Expr>]) -> Self {
        let mut scope = Self::default();
        let mut simplified_exprs = HashMap::new();

        let topo = topological_ordering(roots);
        let usages = count_usages(&topo);

        for expr in &topo {
            let key = ExprKey::from(expr);
            let count = usages.get(&key).copied().unwrap_or(0);

            let simplified_expr = scope.map_expr((**expr).clone(), &simplified_exprs);

            if Self::needs_var(count, expr) {
                let name = var_name(scope.vars.len());

                println!("{name} := {simplified_expr}");

                scope.vars.insert(
                    key,
                    Var {
                        name,
                        expr: simplified_expr.clone(),
                    },
                );
            }

            simplified_exprs.insert(key, simplified_expr);
        }

        for root in roots {
            let key = ExprKey::from(root);
            let simplified_expr = &simplified_exprs[&key];

            println!("root: {simplified_expr}");
        }

        scope
    }
}
