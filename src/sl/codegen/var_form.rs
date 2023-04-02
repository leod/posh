use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::sl::dag::{Expr, Type};

use super::{
    simplified_expr::{ExprKey, SimplifiedExpr, VarId},
    struct_registry::StructRegistry,
};

#[derive(Default)]
pub struct VarForm {
    var_exprs: Vec<SimplifiedExpr>,
    simplified_exprs: HashMap<ExprKey, SimplifiedExpr>,
    roots: Vec<ExprKey>,
}

impl VarForm {
    pub fn new(struct_registry: &StructRegistry, roots: &[Rc<Expr>]) -> Self {
        let mut var_form = Self {
            roots: roots.iter().map(ExprKey::from).collect(),
            ..Self::default()
        };

        let topo = topological_ordering(roots);
        let usages = count_usages(&topo);

        for expr in &topo {
            let key = ExprKey::from(expr);
            let count = usages.get(&key).copied().unwrap_or(0);

            let simplified_expr = var_form.map_expr(struct_registry, (**expr).clone());

            if Self::should_have_var(count, expr) && Self::can_have_var(expr) {
                let var_id = VarId(var_form.var_exprs.len());

                var_form.var_exprs.push(simplified_expr);
                var_form.simplified_exprs.insert(
                    key,
                    SimplifiedExpr::Var {
                        id: var_id,
                        ty: expr.ty(),
                    },
                );
            } else {
                var_form.simplified_exprs.insert(key, simplified_expr);
            }
        }

        var_form
    }

    pub fn var_exprs(&self) -> impl DoubleEndedIterator<Item = (VarId, &'_ SimplifiedExpr)> {
        self.var_exprs
            .iter()
            .enumerate()
            .map(|(var_id, expr)| (VarId(var_id), expr))
    }

    pub fn simplified_roots(&self) -> Vec<&SimplifiedExpr> {
        self.roots
            .iter()
            .map(|root| &self.simplified_exprs[root])
            .collect()
    }

    fn map_expr(&self, struct_registry: &StructRegistry, expr: Expr) -> SimplifiedExpr {
        let map_succ = |succ: Rc<Expr>| self.simplified_exprs[&ExprKey::from(&succ)].clone();

        match expr {
            Expr::Arg { name, ty } => SimplifiedExpr::Arg { name, ty },
            Expr::ScalarLiteral { value, ty } => SimplifiedExpr::ScalarLiteral { value, ty },
            Expr::StructLiteral { args, ty } => SimplifiedExpr::CallFunc {
                name: struct_registry.name(&ty),
                args: args.into_iter().map(map_succ).collect(),
                ty: Type::Struct(ty),
            },
            Expr::ArrayLiteral { args, ty } => SimplifiedExpr::CallFunc {
                name: format!("{}", ty),
                args: args.into_iter().map(map_succ).collect(),
                ty: Type::Array(ty),
            },
            Expr::Unary { op, arg, ty } => SimplifiedExpr::Unary {
                op,
                arg: Box::new(map_succ(arg)),
                ty,
            },
            Expr::Binary {
                left,
                op,
                right,
                ty,
            } => SimplifiedExpr::Binary {
                left: Box::new(map_succ(left)),
                op,
                right: Box::new(map_succ(right)),
                ty,
            },
            Expr::CallFuncDef { def, args } => {
                // TODO: Resolve function name through a prebuilt registry.
                SimplifiedExpr::CallFunc {
                    name: def.name.to_string(),
                    args: args.into_iter().map(map_succ).collect(),
                    ty: def.result.ty(),
                }
            }
            Expr::CallBuiltIn { name, args, ty } => SimplifiedExpr::CallFunc {
                name,
                args: args.into_iter().map(map_succ).collect(),
                ty,
            },
            Expr::Field { base, name, ty } => SimplifiedExpr::Field {
                base: Box::new(map_succ(base)),
                name,
                ty,
            },
            Expr::Subscript { base, index, ty } => SimplifiedExpr::Subscript {
                base: Box::new(map_succ(base)),
                index: Box::new(map_succ(index)),
                ty,
            },
            Expr::Branch { cond, yes, no, ty } => SimplifiedExpr::Branch {
                cond: Box::new(map_succ(cond)),
                yes: Box::new(map_succ(yes)),
                no: Box::new(map_succ(no)),
                ty,
            },
        }
    }

    fn should_have_var(count: usize, expr: &Expr) -> bool {
        use Expr::*;

        match expr {
            Branch { .. } => true,
            Arg { .. } | ScalarLiteral { .. } => false,
            ArrayLiteral { .. } => true,
            StructLiteral { .. }
            | Unary { .. }
            | Binary { .. }
            | CallFuncDef { .. }
            | CallBuiltIn { .. }
            | Subscript { .. }
            | Field { .. } => count > 1,
        }
    }

    fn can_have_var(expr: &Expr) -> bool {
        expr.ty().is_transparent()
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
        panic!("expression contains cycle");
    }

    temporary_mark.insert(key);

    node.successors(|succ| visit(succ, permanent_mark, temporary_mark, output));

    temporary_mark.remove(&key);
    permanent_mark.insert(key);
    output.push(node.clone());
}

fn topological_ordering(roots: &[Rc<Expr>]) -> Vec<Rc<Expr>> {
    let mut permanent_mark = HashSet::new();
    let mut temporary_mark = HashSet::new();
    let mut output = Vec::new();

    for root in roots {
        visit(root, &mut permanent_mark, &mut temporary_mark, &mut output);
    }

    output
}

fn count_usages(exprs: &[Rc<Expr>]) -> HashMap<ExprKey, usize> {
    let mut usages = HashMap::new();

    for expr in exprs {
        expr.successors(|succ| {
            *usages.entry(succ.into()).or_insert(0) += 1;
        })
    }

    usages
}
