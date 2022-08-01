pub mod show;

use std::{collections::HashMap, rc::Rc};

use crate::lang::{
    show::show_ty, BinaryExpr, BranchExpr, BuiltInFunc, CallExpr, Expr, FieldExpr, Func, FuncDef,
    FuncParam, Ident, NamedTy, StructTy, Ty, VarExpr,
};

#[derive(Debug, Clone)]
pub struct ScopedVarDef {
    expr_ptr: *const Expr,
    name: String,
    init: Init,
}

impl ScopedVarDef {
    pub fn expr(&self) -> Expr {
        Expr::Var(VarExpr {
            ident: Ident::new(self.name.clone()),
            ty: self.init.expr().ty(),
        })
    }
}

fn expr_ptr(expr: &Rc<Expr>) -> *const Expr {
    Rc::as_ptr(expr)
}

#[derive(Debug, Clone)]
pub struct ScopedFuncDef {
    name: String,
    params: Vec<FuncParam>,
    scope: Scope,
    result: String,
    result_ty: Ty,
}

#[derive(Debug, Clone)]
pub enum Init {
    Branch(BranchInit),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct BranchInit {
    branch_expr: BranchExpr,
    true_scope: Scope,
    false_scope: Scope,
}

impl Init {
    pub fn expr(&self) -> Expr {
        match self {
            Init::Branch(branch) => Expr::Branch(branch.branch_expr.clone()),
            Init::Expr(expr) => expr.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Defs {
    struct_defs: Vec<StructTy>,
    struct_map: HashMap<StructTy, NamedTy>,

    func_defs: Vec<ScopedFuncDef>,
    func_map: HashMap<FuncDef, BuiltInFunc>,
}

impl Defs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_func_def(func_def: &FuncDef) -> Defs {
        let mut defs = Defs::new();
        defs.add_func_def(func_def);

        defs
    }

    pub fn func_defs(&self) -> impl Iterator<Item = &ScopedFuncDef> {
        self.func_defs.iter()
    }

    pub fn struct_defs(&self) -> impl Iterator<Item = &StructTy> {
        self.struct_defs.iter()
    }

    pub fn add_func_def(&mut self, func: &FuncDef) -> BuiltInFunc {
        let mut params = Vec::new();

        for param in &func.params {
            params.push(FuncParam {
                ident: param.ident.clone(),
                ty: self.walk_ty(&param.ty),
            })
        }

        let mut func_scope = Scope::new();

        let (result_expr, result_name) = func_scope.add_expr(&func.result, self);
        let result_ty = self.walk_ty(&result_expr.ty());

        let name = format!("{}_posh_func_{}", func.ident.name, self.func_defs.len());

        let func_def = FuncDef {
            ident: Ident::new(name.clone()),
            params: params.clone(),
            result: Rc::new(result_expr),
        };

        if let Some(named_func) = self.func_map.get(&func_def) {
            named_func.clone()
        } else {
            let scoped_func_def = ScopedFuncDef {
                name: name.clone(),
                params,
                scope: func_scope,
                result: result_name,
                result_ty: result_ty.clone(),
            };

            // FIXME
            let named_func = BuiltInFunc {
                name,
                ty: result_ty,
            };

            self.func_defs.push(scoped_func_def);
            self.func_map.insert(func_def, named_func.clone());

            named_func
        }
    }

    fn walk_ty(&mut self, ty: &Ty) -> Ty {
        match ty {
            ty @ Ty::BuiltIn(_) => ty.clone(),
            Ty::Struct(ty) => {
                if let Some(named_ty) = self.struct_map.get(ty) {
                    Ty::Named(named_ty.clone())
                } else {
                    let fields = ty
                        .fields
                        .iter()
                        .map(|(field_name, field_ty)| (field_name.clone(), self.walk_ty(field_ty)))
                        .collect();

                    let name = format!("{}_posh_ty_{}", ty.ident.name, self.struct_defs.len());

                    let ident = Ident::new(name.clone());

                    self.struct_defs.push(StructTy { ident, fields });

                    let named_ty = NamedTy { name };
                    self.struct_map.insert(ty.clone(), named_ty.clone());

                    Ty::Named(named_ty)
                }
            }
            ty @ Ty::Named(_) => ty.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Scope {
    next_var_num: usize,
    var_defs: Vec<ScopedVarDef>,
}

impl Scope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn var_defs(&self) -> impl Iterator<Item = (&str, &Init)> {
        self.var_defs
            .iter()
            .map(|var| (var.name.as_str(), &var.init))
    }

    pub fn get_var_def(&self, expr_ptr: *const Expr) -> Option<&ScopedVarDef> {
        self.var_defs.iter().find(|var| var.expr_ptr == expr_ptr)
    }

    pub fn contains_var_def(&self, expr_ptr: *const Expr) -> bool {
        self.get_var_def(expr_ptr).is_some()
    }

    pub fn add_expr(&mut self, expr: &Rc<Expr>, defs: &mut Defs) -> (Expr, String) {
        let expr = self.walk_expr(expr, &[], defs);
        let name = self.var_defs.last().unwrap().name.clone();

        (expr, name)
    }

    fn add_var_def(&mut self, expr: &Rc<Expr>, init: Init) -> Expr {
        let name = format!("posh_var_{}", self.next_var_num);
        self.next_var_num += 1;

        let var_def = ScopedVarDef {
            expr_ptr: Rc::as_ptr(expr),
            name: name.clone(),
            init,
        };
        self.var_defs.push(var_def);

        let ident = Ident::new(name);

        Expr::Var(VarExpr {
            ident,
            ty: expr.ty(),
        })
    }

    fn remove_var_def(&mut self, expr_ptr: *const Expr) {
        let len = self.var_defs.len();
        self.var_defs.retain(|var| var.expr_ptr != expr_ptr);

        assert!(self.var_defs.len() < len);
    }

    fn shared_var_defs<'a>(&'a self, other: &'a Scope) -> HashMap<*const Expr, ScopedVarDef> {
        self.var_defs
            .iter()
            .filter(|var_def| other.contains_var_def(var_def.expr_ptr))
            .chain(
                other
                    .var_defs
                    .iter()
                    .filter(|var_def| self.contains_var_def(var_def.expr_ptr)),
            )
            .map(|var_def| (var_def.expr_ptr, var_def.clone()))
            .collect()
    }

    fn walk_child_expr(
        &mut self,
        child_expr: &Rc<Expr>,
        parents: &[&Scope],
        defs: &mut Defs,
    ) -> (Scope, Expr) {
        let mut child_parents: Vec<&Scope> = parents.to_vec();
        child_parents.push(self);

        let mut child_scope = Self {
            next_var_num: self.next_var_num,
            ..Self::default()
        };

        let child_expr = child_scope.walk_expr(child_expr, &child_parents, defs);

        self.next_var_num = child_scope.next_var_num;

        (child_scope, child_expr)
    }

    fn walk_expr(&mut self, expr: &Rc<Expr>, parents: &[&Scope], defs: &mut Defs) -> Expr {
        use Expr::*;

        match &**expr {
            expr @ Var(_) => return expr.clone(),
            expr @ Literal(_) => return expr.clone(),
            _ => (),
        }

        if let Some(var_def) = self.get_var_def(expr_ptr(expr)) {
            return var_def.expr();
        }

        for parent in parents {
            if let Some(var_def) = parent.get_var_def(expr_ptr(expr)) {
                return var_def.expr();
            }
        }

        let init = match &**expr {
            Binary(expr) => {
                let left = self.walk_expr(&expr.left, parents, defs);
                let right = self.walk_expr(&expr.right, parents, defs);

                Init::Expr(Expr::Binary(BinaryExpr {
                    left: Rc::new(left),
                    op: expr.op,
                    right: Rc::new(right),
                    ty: defs.walk_ty(&expr.ty),
                }))
            }
            Branch(expr) => {
                let cond = self.walk_expr(&expr.cond, parents, defs);
                let (mut true_scope, true_expr) =
                    self.walk_child_expr(&expr.true_expr, parents, defs);

                // FIXME: Needs to be able to pull out variables from `true_expr`.
                let (mut false_scope, false_expr) =
                    self.walk_child_expr(&expr.false_expr, parents, defs);

                // FIXME: Produces references to removed variables.
                for (expr_ptr, var_def) in true_scope.shared_var_defs(&false_scope) {
                    true_scope.remove_var_def(expr_ptr);
                    false_scope.remove_var_def(expr_ptr);

                    assert!(!self.contains_var_def(expr_ptr));
                    self.var_defs.push(var_def.clone());
                }

                let branch_expr = BranchExpr {
                    cond: Rc::new(cond),
                    true_expr: Rc::new(true_expr),
                    false_expr: Rc::new(false_expr),
                };

                Init::Branch(BranchInit {
                    branch_expr,
                    true_scope,
                    false_scope,
                })
            }
            Var(_) => unreachable!(),
            Call(expr) => {
                use Func::*;

                let mut args = Vec::new();

                for arg in &expr.args {
                    args.push(Rc::new(self.walk_expr(arg, parents, defs)));
                }

                let func = match &expr.func {
                    func @ BuiltIn(_) => func.clone(),
                    Def(func) => Func::BuiltIn(defs.add_func_def(func)),
                    Struct(func) => {
                        let ty = defs.walk_ty(&Ty::Struct(func.ty.clone()));

                        // FIXME
                        BuiltIn(BuiltInFunc {
                            name: show_ty(&ty),
                            ty,
                        })
                    }
                };

                Init::Expr(Expr::Call(CallExpr { func, args }))
            }
            Literal(_) => unreachable!(),
            Field(expr) => {
                let base = self.walk_expr(&expr.base, parents, defs);

                Init::Expr(Expr::Field(FieldExpr {
                    base: Rc::new(base),
                    member: expr.member.clone(),
                    ty: defs.walk_ty(&expr.ty),
                }))
            }
        };

        self.add_var_def(expr, init)
    }
}
