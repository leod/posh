use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::dag::Expr;

use super::ExprKey;

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
    output: &mut Vec<ExprKey>,
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

    temporary_mark.remove(&key);
    permanent_mark.insert(key);
    output.push(key);
}

pub fn topological_ordering(roots: impl Iterator<Item = Rc<Expr>>) -> HashMap<ExprKey, usize> {
    let mut permanent_mark = HashSet::new();
    let mut temporary_mark = HashSet::new();
    let mut output = Vec::new();

    for root in roots {
        visit(&root, &mut permanent_mark, &mut temporary_mark, &mut output);
    }

    output
        .into_iter()
        .rev()
        .enumerate()
        .map(|(index, key)| (key, index))
        .collect()
}
