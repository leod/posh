pub mod show;

use std::{collections::HashMap, mem, rc::Rc};

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
pub struct Scope {
    struct_defs: Vec<StructTy>,
    struct_def_map: HashMap<StructTy, NamedTy>,

    func_defs: Vec<ScopedFuncDef>,
    func_def_map: HashMap<FuncDef, BuiltInFunc>,

    next_var_num: usize,
    var_defs: Vec<ScopedVarDef>,
}

impl Scope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_func_def(func_def: &FuncDef) -> Self {
        let mut scope = Scope::new();
        scope.add_func_def(func_def);

        scope
    }

    pub fn var_defs(&self) -> impl Iterator<Item = (&str, &Init)> {
        self.var_defs
            .iter()
            .map(|var| (var.name.as_str(), &var.init))
    }

    pub fn func_defs(&self) -> impl Iterator<Item = &ScopedFuncDef> {
        self.func_defs.iter()
    }

    pub fn struct_defs(&self) -> impl Iterator<Item = &StructTy> {
        self.struct_defs.iter()
    }

    pub fn get_var_def(&self, expr_ptr: *const Expr) -> Option<&ScopedVarDef> {
        self.var_defs.iter().find(|var| var.expr_ptr == expr_ptr)
    }

    pub fn contains_var_def(&self, expr_ptr: *const Expr) -> bool {
        self.get_var_def(expr_ptr).is_some()
    }

    pub fn add_expr(&mut self, expr: &Rc<Expr>) -> (Expr, String) {
        let expr = self.walk_expr(expr, &[]);
        let name = self.var_defs.last().unwrap().name.clone();

        (expr, name)
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
        func_scope.func_defs = mem::replace(&mut self.func_defs, Default::default());
        func_scope.struct_defs = mem::replace(&mut self.struct_defs, Default::default());

        let (result_expr, result_name) = func_scope.add_expr(&func.result);
        let result_ty = self.walk_ty(&result_expr.ty());

        self.func_defs = mem::replace(&mut func_scope.func_defs, Default::default());
        self.struct_defs = mem::replace(&mut func_scope.struct_defs, Default::default());

        let name = format!("{}_posh_func_{}", func.ident.name, self.func_defs.len());

        let func_def = FuncDef {
            ident: Ident::new(name.clone()),
            params: params.clone(),
            result: Rc::new(result_expr),
        };

        if let Some(named_func) = self.func_def_map.get(&func_def) {
            named_func.clone()
        } else {
            let scoped_func_def = ScopedFuncDef {
                name: name.clone(),
                params,
                scope: func_scope,
                result: result_name,
                result_ty: result_ty.clone(),
            };

            let named_func = BuiltInFunc {
                name,
                ty: result_ty,
            };

            self.func_defs.push(scoped_func_def.clone());
            self.func_def_map.insert(func_def, named_func.clone());

            named_func
        }
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

    fn walk_ty(&mut self, ty: &Ty) -> Ty {
        match ty {
            ty @ Ty::BuiltIn(_) => ty.clone(),
            Ty::Struct(ty) => {
                if let Some(named_ty) = self.struct_def_map.get(ty) {
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
                    self.struct_def_map.insert(ty.clone(), named_ty.clone());

                    Ty::Named(named_ty)
                }
            }
            ty @ Ty::Named(_) => ty.clone(),
        }
    }

    fn remove_var_def(&mut self, expr_ptr: *const Expr) {
        let len = self.var_defs.len();
        self.var_defs.retain(|var| var.expr_ptr != expr_ptr);

        assert!(self.var_defs.len() < len);
    }

    fn shared_var_defs(&self, other: &Scope) -> Vec<ScopedVarDef> {
        self.var_defs
            .iter()
            .filter(|var| other.contains_var_def(var.expr_ptr))
            .chain(
                other
                    .var_defs
                    .iter()
                    .filter(|var| self.contains_var_def(var.expr_ptr)),
            )
            .cloned()
            .collect()
    }

    fn walk_child_expr(&mut self, child_expr: &Rc<Expr>, parents: &[&Scope]) -> (Scope, Expr) {
        let mut child_parents: Vec<&Scope> = parents.iter().map(|x| *x).collect();
        child_parents.push(self);

        let mut child_scope = Self {
            next_var_num: self.next_var_num,
            ..Self::default()
        };

        let child_expr = child_scope.walk_expr(child_expr, &child_parents);

        self.next_var_num = child_scope.next_var_num;

        (child_scope, child_expr)
    }

    fn walk_expr(&mut self, expr: &Rc<Expr>, parents: &[&Scope]) -> Expr {
        use Expr::*;

        if let Some(var) = self.get_var_def(expr_ptr(expr)) {
            return var.init.expr();
        }

        for parent in parents {
            if let Some(var) = parent.get_var_def(expr_ptr(expr)) {
                return var.init.expr();
            }
        }

        let init = match &**expr {
            Binary(expr) => {
                let left = self.walk_expr(&expr.left, parents);
                let right = self.walk_expr(&expr.left, parents);

                Init::Expr(Expr::Binary(BinaryExpr {
                    left: Rc::new(left),
                    op: expr.op,
                    right: Rc::new(right),
                    ty: self.walk_ty(&expr.ty),
                }))
            }
            Branch(expr) => {
                let cond = self.walk_expr(&expr.cond, parents);
                let (mut true_scope, true_expr) = self.walk_child_expr(&expr.true_expr, parents);
                let (mut false_scope, false_expr) = self.walk_child_expr(&expr.false_expr, parents);

                for var_def in true_scope.shared_var_defs(&false_scope) {
                    true_scope.remove_var_def(var_def.expr_ptr);
                    false_scope.remove_var_def(var_def.expr_ptr);

                    assert!(!self.contains_var_def(var_def.expr_ptr));
                    self.var_defs.push(var_def);
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
            Var(expr) => Init::Expr(Expr::Var(expr.clone())),
            Call(expr) => {
                use Func::*;

                let mut args = Vec::new();

                for arg in &expr.args {
                    args.push(Rc::new(self.walk_expr(arg, parents)));
                }

                let func = match &expr.func {
                    func @ BuiltIn(_) => func.clone(),
                    Def(func) => Func::BuiltIn(self.add_func_def(func)),
                    Struct(func) => {
                        let ty = self.walk_ty(&Ty::Struct(func.ty.clone()));

                        // FIXME
                        BuiltIn(BuiltInFunc {
                            name: show_ty(&ty),
                            ty,
                        })
                    }
                };

                Init::Expr(Expr::Call(CallExpr { func, args }))
            }
            expr @ Literal(_) => Init::Expr(expr.clone()),
            Field(expr) => {
                let base = self.walk_expr(&expr.base, parents);

                Init::Expr(Expr::Field(FieldExpr {
                    base: Rc::new(base),
                    member: expr.member.clone(),
                    ty: self.walk_ty(&expr.ty),
                }))
            }
        };

        self.add_var_def(expr, init)
    }
}
