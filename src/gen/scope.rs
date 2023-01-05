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
    pub fn map_expr(&self, expr: Expr, mapped_exprs: &HashMap<ExprKey, Expr>) -> Expr {
        use Expr::*;

        let lookup = |node: Rc<Expr>| {
            let key = ExprKey::from(&node);
            self.vars.get(&key).map_or_else(
                || {
                    mapped_exprs
                        .get(&key)
                        .cloned()
                        .map_or(node, |mapped_expr| Rc::new(mapped_expr))
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

    pub fn boop(roots: &[Rc<Expr>]) -> Self {
        let mut scope = Self::default();
        let mut mapped_exprs = HashMap::new();

        let topo = topological_ordering(roots);
        let usages = count_usages(&topo);

        for expr in &topo {
            let key = ExprKey::from(expr);
            let count = usages.get(&key).copied().unwrap_or(0);

            let mapped_expr = scope.map_expr((**expr).clone(), &mapped_exprs);

            if count > 1 {
                let name = var_name(scope.vars.len());

                println!("{name} := {mapped_expr}");

                scope.vars.insert(
                    key,
                    Var {
                        name,
                        expr: mapped_expr.clone(),
                    },
                );
            } else {
                println!("{mapped_expr}");
            }

            mapped_exprs.insert(key, mapped_expr);
        }

        scope
    }
}
