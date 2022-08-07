use std::collections::BTreeSet;

use super::{Expr, Func, FuncDef, StructTy, Ty, VarExpr};

#[derive(Debug, Clone, Default)]
pub struct Defs {
    pub structs: BTreeSet<StructTy>,
    pub funcs: BTreeSet<FuncDef>,
}

impl Defs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_expr(expr: &Expr) -> Self {
        let mut structs = BTreeSet::new();
        let mut funcs = BTreeSet::new();

        collect_structs(expr, &mut structs);
        collect_funcs(expr, &mut funcs);

        Self { structs, funcs }
    }

    pub fn from_func_def(func: &FuncDef) -> Self {
        let mut defs = Defs::from_expr(&func.result);
        defs.funcs.insert(func.clone());

        defs
    }

    pub fn extend(&mut self, defs: &Defs) {
        self.structs.extend(defs.structs.clone());
        self.funcs.extend(defs.funcs.clone());
    }

    pub fn close(&mut self) {}
}

pub fn collect_structs(expr: &Expr, structs: &mut BTreeSet<StructTy>) {
    use Expr::*;

    collect_struct_ty(&expr.ty(), structs);

    match expr {
        Binary(expr) => {
            collect_structs(&*expr.left, structs);
            collect_structs(&*expr.right, structs);
        }
        Branch(expr) => {
            collect_structs(&*expr.cond, structs);
            collect_structs(&*expr.true_expr, structs);
            collect_structs(&*expr.false_expr, structs);
        }
        Var(_) => (),
        Call(expr) => {
            if let Func::Def(func) = &expr.func {
                for (_, param_ty) in func.params.iter() {
                    collect_struct_ty(param_ty, structs);
                }
                collect_structs(&*func.result, structs);
            }

            for arg in &expr.args {
                collect_structs(arg, structs);
            }
        }
        Literal(_) => (),
        Field(expr) => {
            collect_structs(&*expr.base, structs);
        }
    }
}

fn collect_struct_ty(ty: &Ty, structs: &mut BTreeSet<StructTy>) {
    if let Ty::Struct(ref ty) = ty {
        structs.insert(ty.clone());

        for (name, ty) in ty.fields.iter() {
            collect_structs(
                &Expr::Var(VarExpr {
                    name: name.clone(),
                    ty: ty.clone(),
                }),
                structs,
            );
        }
    }
}

pub fn collect_funcs(expr: &Expr, funcs: &mut BTreeSet<FuncDef>) {
    use Expr::*;

    match expr {
        Binary(expr) => {
            collect_funcs(&*expr.left, funcs);
            collect_funcs(&*expr.right, funcs);
        }
        Branch(expr) => {
            collect_funcs(&*expr.cond, funcs);
            collect_funcs(&*expr.true_expr, funcs);
            collect_funcs(&*expr.false_expr, funcs);
        }
        Var(_) => (),
        Call(expr) => {
            if let Func::Def(func) = &expr.func {
                funcs.insert(func.clone());
                collect_funcs(&*func.result, funcs);
            }
            for arg in &expr.args {
                collect_funcs(arg, funcs);
            }
        }
        Literal(_) => (),
        Field(expr) => {
            collect_funcs(&*expr.base, funcs);
        }
    }
}
