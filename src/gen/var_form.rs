use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::dag::{BaseType, BinaryOp, Expr, PrimitiveType, Type};

use super::ExprKey;

#[derive(Debug, Clone)]
pub enum SimplifiedExpr {
    Branch {
        cond: Box<SimplifiedExpr>,
        yes: Box<SimplifiedExpr>,
        no: Box<SimplifiedExpr>,
        ty: Type,
    },
    Arg {
        name: String,
        ty: Type,
    },
    ScalarLiteral {
        value: String,
        ty: PrimitiveType,
    },
    Binary {
        left: Box<SimplifiedExpr>,
        op: BinaryOp,
        right: Box<SimplifiedExpr>,
        ty: Type,
    },
    CallFunc {
        name: String,
        args: Vec<SimplifiedExpr>,
        ty: Type,
    },
    Field {
        base: Box<SimplifiedExpr>,
        name: &'static str,
        ty: Type,
    },
    Var {
        id: VarId,
        ty: Type,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarId(usize);

impl VarId {
    pub fn index(self) -> usize {
        self.0
    }
}

#[derive(Default)]
pub struct VarForm {
    var_exprs: Vec<SimplifiedExpr>,
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
                let var_id = VarId(var_form.var_exprs.len());

                println!("{:?} {} = {}", var_id, expr.ty(), simplified_expr);

                var_form.var_exprs.push(simplified_expr);
                var_form.expr_to_var.insert(key, var_id);
            } else {
                var_form.simplified_exprs.insert(key, simplified_expr);
            }
        }

        for root in roots {
            let key = ExprKey::from(root);
            let simplified_expr = &var_form.simplified_exprs[&key];

            println!("{}", simplified_expr);
        }

        var_form
    }

    pub fn var_exprs(&self) -> &[SimplifiedExpr] {
        &self.var_exprs
    }

    fn map_expr(&self, expr: Expr) -> SimplifiedExpr {
        let map_pred = |pred: Rc<Expr>| {
            let key = ExprKey::from(&pred);

            if let Some(&var_id) = self.expr_to_var.get(&key) {
                SimplifiedExpr::Var {
                    id: var_id,
                    ty: pred.ty(),
                }
            } else {
                self.simplified_exprs[&key].clone()
            }
        };

        match expr {
            Expr::Arg { name, ty } => SimplifiedExpr::Arg { name, ty },
            Expr::ScalarLiteral { value, ty } => SimplifiedExpr::ScalarLiteral { value, ty },
            Expr::StructLiteral { args, ty } => {
                // TODO: Resolve struct name through a prebuilt registry.
                SimplifiedExpr::CallFunc {
                    name: ty.name.to_string(),
                    args: args.into_iter().map(map_pred).collect(),
                    ty: Type::Base(BaseType::Struct(ty)),
                }
            }
            Expr::Binary {
                left,
                op,
                right,
                ty,
            } => SimplifiedExpr::Binary {
                left: Box::new(map_pred(left)),
                op,
                right: Box::new(map_pred(right)),
                ty,
            },
            Expr::CallFuncDef { def, args } => {
                // TODO: Resolve function name through a prebuilt registry.
                SimplifiedExpr::CallFunc {
                    name: def.name.to_string(),
                    args: args.into_iter().map(map_pred).collect(),
                    ty: def.result.ty(),
                }
            }
            Expr::CallBuiltIn { name, args, ty } => SimplifiedExpr::CallFunc {
                name: name.to_string(),
                args: args.into_iter().map(map_pred).collect(),
                ty,
            },
            Expr::Field { base, name, ty } => SimplifiedExpr::Field {
                base: Box::new(map_pred(base)),
                name: name,
                ty,
            },
            Expr::Branch { cond, yes, no, ty } => SimplifiedExpr::Branch {
                cond: Box::new(map_pred(cond)),
                yes: Box::new(map_pred(yes)),
                no: Box::new(map_pred(no)),
                ty,
            },
        }

        /*let simplified_expr = match expr {
            expr @ Expr::Arg { .. } | expr @ ScalarLiteral { .. } => expr,
            StructLiteral { args, ty } => StructLiteral {
                args: args.into_iter().map(map_pred).collect(),
                ty,
            },
            Binary {
                left,
                op,
                right,
                ty,
            } => Binary {
                left: map_pred(left),
                op,
                right: map_pred(right),
                ty,
            },
            CallFuncDef { .. } => todo!(),
            CallBuiltIn { name, args, ty } => CallBuiltIn {
                name,
                args: args.into_iter().map(map_pred).collect(),
                ty,
            },
            Field { base, name, ty } => Field {
                base: map_pred(base),
                name,
                ty,
            },
            Branch { cond, yes, no, ty } => Branch {
                cond: map_pred(cond),
                yes: map_pred(yes),
                no: map_pred(no),
                ty,
            },
        };

        SimplifiedExpr {
            expr: simplified_expr,
            deps,
        }*/
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
