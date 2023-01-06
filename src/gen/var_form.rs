use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::dag::Expr;

use super::ExprKey;

#[derive(Debug, Clone)]
pub struct Var {
    name: String,
    expr: Expr,
}

#[derive(Default)]
pub struct VarForm {
    vars: HashMap<ExprKey, Var>,
}

fn var_name(idx: usize) -> String {
    format!("var_{idx}")
}

fn map_expr(expr: Expr, map: &HashMap<ExprKey, Expr>) -> Expr {
    use Expr::*;

    let lookup = |node: Rc<Expr>| {
        let key = ExprKey::from(&node);
        map.get(&key)
            .cloned()
            .map_or(node, |mapped_expr| Rc::new(mapped_expr))
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

pub fn count_usages(exprs: &[Rc<Expr>]) -> HashMap<ExprKey, usize> {
    let mut usages = HashMap::new();

    for expr in exprs {
        predecessors(expr, |pred| {
            *usages.entry(pred.into()).or_insert(0) += 1;
        })
    }

    usages
}

impl VarForm {
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

    pub fn new(roots: &[Rc<Expr>]) -> Self {
        let mut scope = Self::default();
        let mut simplified_exprs = HashMap::new();

        let topo = topological_ordering(roots);
        let usages = count_usages(&topo);

        for expr in &topo {
            let key = ExprKey::from(expr);
            let count = usages.get(&key).copied().unwrap_or(0);

            let simplified_expr = map_expr((**expr).clone(), &simplified_exprs);

            if Self::needs_var(count, expr) {
                let name = var_name(scope.vars.len());

                println!("{} {} = {}", expr.ty(), name, simplified_expr);

                let var_expr = Var {
                    name: name.clone(),
                    expr: simplified_expr.clone(),
                };

                scope.vars.insert(key, var_expr.clone());
                simplified_exprs.insert(
                    key,
                    Expr::Arg {
                        name,
                        ty: simplified_expr.ty(),
                    },
                );
            } else {
                simplified_exprs.insert(key, simplified_expr);
            }
        }

        for root in roots {
            let key = ExprKey::from(root);
            let simplified_expr = &simplified_exprs[&key];

            println!("{simplified_expr}");
        }

        scope
    }
}
