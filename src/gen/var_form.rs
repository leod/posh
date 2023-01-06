use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::dag::Expr;

use super::ExprKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarId(usize);

#[derive(Debug, Clone)]
pub struct Var {
    pub name: String,
    pub simplified_expr: SimplifiedExpr,
}

#[derive(Debug, Clone)]
pub struct SimplifiedExpr {
    pub expr: Expr,
    pub deps: HashSet<VarId>,
}

#[derive(Default)]
pub struct VarForm {
    vars: Vec<Var>,
    expr_to_var: HashMap<ExprKey, VarId>,
    simplified_exprs: HashMap<ExprKey, SimplifiedExpr>,
}

impl VarForm {
    pub fn new(roots: &[Rc<Expr>]) -> Self {
        let mut var_form = Self::default();

        let topo = topological_ordering(roots);
        let usages = count_usages(&topo);

        for expr in &topo {
            let key = ExprKey::from(expr);
            let count = usages.get(&key).copied().unwrap_or(0);

            let simplified_expr = var_form.map_expr((**expr).clone());

            if Self::needs_var(count, expr) {
                let name = var_name(var_form.expr_to_var.len());

                let var_id = VarId(var_form.vars.len());
                let var = Var {
                    name: name.clone(),
                    simplified_expr: simplified_expr.clone(),
                };

                println!(
                    "{} {} = {}: {:?} @ {:?}",
                    expr.ty(),
                    name,
                    simplified_expr.expr,
                    simplified_expr.deps,
                    var_id,
                );

                var_form.vars.push(var);
                var_form.expr_to_var.insert(key, var_id);
                var_form.simplified_exprs.insert(
                    key,
                    SimplifiedExpr {
                        expr: Expr::Arg {
                            name,
                            ty: simplified_expr.expr.ty(),
                        },
                        deps: simplified_expr.deps,
                    },
                );
            } else {
                var_form.simplified_exprs.insert(key, simplified_expr);
            }
        }

        for root in roots {
            let key = ExprKey::from(root);
            let simplified_expr = &var_form.simplified_exprs[&key];

            println!("{}: {:?}", simplified_expr.expr, simplified_expr.deps);
        }

        var_form
    }

    fn map_expr(&self, expr: Expr) -> SimplifiedExpr {
        use Expr::*;

        let mut deps = HashSet::new();

        let mut handle_pred = |pred: Rc<Expr>| {
            let key = ExprKey::from(&pred);

            if let Some(&var_id) = self.expr_to_var.get(&key) {
                deps.insert(var_id);

                Rc::new(Expr::Arg {
                    name: self.vars[var_id.0].name.clone(),
                    ty: pred.ty(),
                })
            } else if let Some(simplified_pred) = self.simplified_exprs.get(&key) {
                deps.extend(simplified_pred.deps.iter().copied());

                Rc::new(simplified_pred.expr.clone())
            } else {
                pred.clone()
            }
        };

        let simplified_expr = match expr {
            expr @ Arg { .. } | expr @ ScalarLiteral { .. } => expr,
            StructLiteral { args, ty } => StructLiteral {
                args: args.into_iter().map(handle_pred).collect(),
                ty,
            },
            Binary {
                left,
                op,
                right,
                ty,
            } => Binary {
                left: handle_pred(left),
                op,
                right: handle_pred(right),
                ty,
            },
            CallFuncDef { .. } => todo!(),
            CallBuiltIn { name, args, ty } => CallBuiltIn {
                name,
                args: args.into_iter().map(handle_pred).collect(),
                ty,
            },
            Field { base, name, ty } => Field {
                base: handle_pred(base),
                name,
                ty,
            },
            Branch { cond, yes, no, ty } => Branch {
                cond: handle_pred(cond),
                yes: handle_pred(yes),
                no: handle_pred(no),
                ty,
            },
        };

        SimplifiedExpr {
            expr: simplified_expr,
            deps,
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
}

fn var_name(idx: usize) -> String {
    format!("var_{idx}")
}

fn predecessors(expr: &Expr, mut f: impl FnMut(&Rc<Expr>)) {
    use Expr::*;

    match expr {
        Arg { .. } => (),
        ScalarLiteral { .. } => (),
        StructLiteral { args, .. } => {
            for arg in args {
                f(arg);
            }
        }
        Binary { left, right, .. } => {
            f(left);
            f(right);
        }
        CallFuncDef { args, .. } => {
            for arg in args {
                f(arg);
            }
        }
        CallBuiltIn { args, .. } => {
            for arg in args {
                f(arg);
            }
        }
        Field { base, .. } => {
            f(base);
        }
        Branch { cond, yes, no, .. } => {
            f(cond);
            f(yes);
            f(no);
        }
    }
}

fn visit(
    node: &Rc<Expr>,
    permanent_mark: &mut HashSet<ExprKey>,
    temporary_mark: &mut HashSet<ExprKey>,
    output: &mut Vec<Rc<Expr>>,
) {
    let key: ExprKey = node.into();

    if permanent_mark.contains(&key) {
        return;
    }

    if temporary_mark.contains(&key) {
        panic!("Expression contains cycle");
    }

    temporary_mark.insert(key);

    predecessors(node, |pred| {
        visit(pred, permanent_mark, temporary_mark, output)
    });

    //println!("{}: {} @ {:?}", output.len(), node, key);

    temporary_mark.remove(&key);
    permanent_mark.insert(key);
    output.push(node.clone());
}

fn topological_ordering(roots: &[Rc<Expr>]) -> Vec<Rc<Expr>> {
    let mut permanent_mark = HashSet::new();
    let mut temporary_mark = HashSet::new();
    let mut output = Vec::new();

    for root in roots {
        visit(&root, &mut permanent_mark, &mut temporary_mark, &mut output);
    }

    /*output
    .into_iter()
    .enumerate()
    .map(|(index, key)| (key, index))
    .collect()*/

    output
}

fn count_usages(exprs: &[Rc<Expr>]) -> HashMap<ExprKey, usize> {
    let mut usages = HashMap::new();

    for expr in exprs {
        predecessors(expr, |pred| {
            *usages.entry(pred.into()).or_insert(0) += 1;
        })
    }

    usages
}
