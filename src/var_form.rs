pub mod show;
mod struct_defs;

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::lang::{
    show::{show_expr, show_ty},
    BinaryExpr, BranchExpr, CallExpr, Expr, FieldExpr, Func, FuncDef, FuncParam, Ident, NameFunc,
    Ty, VarExpr,
};

pub use struct_defs::StructDefs;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VarId(usize);

pub fn var_name(id: VarId) -> Ident {
    Ident::new(format!("posh_var_{}", id.0))
}

#[derive(Debug, Clone)]
pub enum VarInit {
    Branch(BranchVarInit),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub struct BranchVarInit {
    branch_expr: BranchExpr,
    true_scope: Rc<RefCell<Scope>>,
    false_scope: Rc<RefCell<Scope>>,
}

impl VarInit {
    pub fn expr(&self) -> Expr {
        match self {
            VarInit::Branch(branch) => Expr::Branch(branch.branch_expr.clone()),
            VarInit::Expr(expr) => expr.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Scope {
    depth: usize,
    parent: Option<Rc<RefCell<Scope>>>,
    vars: Vec<(VarId, VarInit)>,
}

type ScopeRef = Rc<RefCell<Scope>>;

impl Scope {
    fn remove(&mut self, remove_id: VarId) -> VarInit {
        let var_init = self
            .vars
            .iter()
            .find(|(id, _)| *id == remove_id)
            .unwrap()
            .1
            .clone();

        self.vars.retain(|(id, _)| *id != remove_id);

        var_init
    }

    fn vars(&self) -> &[(VarId, VarInit)] {
        &self.vars
    }
}

fn new_child_scope(scope: ScopeRef) -> ScopeRef {
    let scope = Scope {
        depth: scope.borrow().depth + 1,
        parent: Some(scope.clone()),
        vars: Vec::new(),
    };

    Rc::new(RefCell::new(scope))
}

#[derive(Debug, Clone)]
pub struct VarFormFunc {
    params: Vec<FuncParam>,
    scope: Rc<RefCell<Scope>>,
    result: (Expr, Ty),
}

#[derive(Debug, Default, Clone)]
pub struct VarFormFuncDefs {
    defs: Vec<(String, VarFormFunc)>,
    map: HashMap<FuncDef, NameFunc>,
}

fn func_name(name: &str, num_defs: usize) -> String {
    format!("{}_posh_ty_{}", name, num_defs)
}

impl VarFormFuncDefs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_func_def(func_def: &FuncDef, structs: &mut StructDefs) -> Self {
        let mut defs = Self::new();
        defs.add(func_def, structs);

        defs
    }

    pub fn defs(&self) -> impl Iterator<Item = &(String, VarFormFunc)> {
        self.defs.iter()
    }

    pub fn add(&mut self, func: &FuncDef, structs: &mut StructDefs) -> NameFunc {
        let mut params = Vec::new();

        for param in &func.params {
            params.push(FuncParam {
                ident: param.ident.clone(),
                ty: structs.walk(&param.ty),
            })
        }

        let scope = Rc::new(RefCell::new(Scope::default()));
        let mut func_scope_builder = ScopeBuilder::default();

        let result_expr = func_scope_builder
            .walk_expr(scope.clone(), func.result.clone(), structs, self)
            .0;
        let result_ty = structs.walk(&result_expr.ty());

        let func_def = FuncDef {
            ident: func.ident.clone(),
            params: params.clone(),
            result: Rc::new(result_expr.clone()),
        };

        if let Some(name_func) = self.map.get(&func_def) {
            name_func.clone()
        } else {
            let var_form_func = VarFormFunc {
                params,
                scope,
                result: (result_expr, result_ty.clone()),
            };

            let name_func = NameFunc {
                name: func_name(&func.ident.name, self.defs.len()),
                ty: result_ty,
            };

            self.defs.push((func.ident.name.clone(), var_form_func));
            self.map.insert(func_def, name_func.clone());

            name_func
        }
    }
}

#[derive(Debug, Clone)]
struct VarInfo {
    scope: ScopeRef,
    deps: HashSet<VarId>,
}

#[derive(Debug, Default, Clone)]
pub struct ScopeBuilder {
    next_var_id: VarId,
    var_ids: HashMap<*const Expr, VarId>,
    var_infos: HashMap<VarId, VarInfo>,
}

fn expr_ptr(expr: &Rc<Expr>) -> *const Expr {
    Rc::as_ptr(expr)
}

struct LCA {
    scope: ScopeRef,
    u_child: Option<ScopeRef>,
    v_child: Option<ScopeRef>,
}

impl LCA {
    fn find(mut u: ScopeRef, mut v: ScopeRef) -> Self {
        let mut u_child = None;
        let mut v_child = None;

        while u.borrow().depth != v.borrow().depth {
            if u.borrow().depth > v.borrow().depth {
                u_child = Some(u.clone());

                let u_parent = u.borrow().parent.clone();
                u = u_parent.expect("invalid depth or parent");
            } else {
                v_child = Some(v.clone());

                let v_parent = v.borrow().parent.clone();
                v = v_parent.expect("invalid depth or parent");
            }
        }

        while !Rc::ptr_eq(&u, &v) {
            u_child = Some(u.clone());
            v_child = Some(v.clone());

            let u_parent = u.borrow().parent.clone();
            u = u_parent.expect("no lca exists");

            let v_parent = v.borrow().parent.clone();
            v = v_parent.expect("no lca exists");
        }

        //assert!(u_child.is_some() || v_child.is_some());

        Self {
            scope: u.clone(),
            u_child,
            v_child,
        }
    }
}

impl ScopeBuilder {
    fn walk_expr(
        &mut self,
        scope: ScopeRef,
        expr: Rc<Expr>,
        structs: &mut StructDefs,
        funcs: &mut VarFormFuncDefs,
    ) -> (Expr, HashSet<VarId>) {
        use Expr::*;

        match &*expr {
            expr @ Var(_) => return (expr.clone(), HashSet::new()),
            expr @ Literal(_) => return (expr.clone(), HashSet::new()),
            _ => (),
        }

        if let Some(var_id) = self.var_ids.get(&expr_ptr(&expr)).cloned() {
            let var_info = self.var_infos[&var_id].clone();

            eprintln!("match {:?}", var_id);

            let lca = LCA::find(scope.clone(), var_info.scope.clone());

            let mut result_deps = var_info.deps.clone();
            result_deps.insert(var_id);

            let result_expr = Var(VarExpr {
                ident: var_name(var_id),
                ty: expr.ty(),
            });

            if !Rc::ptr_eq(&lca.scope, &var_info.scope) {
                eprintln!(
                    "pulling {:?} from {:?} to {:?}",
                    var_id,
                    Rc::as_ptr(&var_info.scope),
                    Rc::as_ptr(&lca.scope)
                );

                for dep in &var_info.deps {
                    let dep_info = &self.var_infos[&dep];
                    let dep_lca = LCA::find(scope.clone(), dep_info.scope.clone());

                    let dep_init = dep_info.scope.borrow_mut().remove(*dep);
                    dep_lca.scope.borrow_mut().vars.push((*dep, dep_init));
                    self.var_infos.get_mut(&dep).unwrap().scope = dep_lca.scope;
                }

                let var_init = var_info.scope.borrow_mut().remove(var_id);
                lca.scope.borrow_mut().vars.push((var_id, var_init));
                self.var_infos.get_mut(&var_id).unwrap().scope = lca.scope;
            }

            return (result_expr, result_deps);
        }

        // FIXME: We will need to turn this recursion into iteration so that we don't stack overflow
        //        on deep expressions.

        let (var_init, mut deps) = match &*expr {
            Binary(expr) => {
                let (left_expr, left_deps) =
                    self.walk_expr(scope.clone(), expr.left.clone(), structs, funcs);
                let (right_expr, right_deps) =
                    self.walk_expr(scope.clone(), expr.right.clone(), structs, funcs);

                let mut deps = left_deps;
                deps.extend(right_deps);

                (
                    VarInit::Expr(Binary(BinaryExpr {
                        left: Rc::new(left_expr),
                        op: expr.op,
                        right: Rc::new(right_expr),
                        ty: structs.walk(&expr.ty),
                    })),
                    deps,
                )
            }
            Branch(expr) => {
                let (cond_expr, cond_deps) =
                    self.walk_expr(scope.clone(), expr.cond.clone(), structs, funcs);

                let true_scope = new_child_scope(scope.clone());
                let (true_expr, mut true_deps) =
                    self.walk_expr(true_scope.clone(), expr.true_expr.clone(), structs, funcs);

                for (var_id, _) in &true_scope.borrow().vars {
                    true_deps.remove(var_id);
                }

                let false_scope = new_child_scope(scope.clone());
                let (false_expr, mut false_deps) =
                    self.walk_expr(false_scope.clone(), expr.false_expr.clone(), structs, funcs);

                for (var_id, _) in &false_scope.borrow().vars {
                    false_deps.remove(var_id);
                }

                let mut deps = cond_deps;
                deps.extend(true_deps);
                deps.extend(false_deps);

                let branch_expr = BranchExpr {
                    cond: Rc::new(cond_expr),
                    true_expr: Rc::new(true_expr),
                    false_expr: Rc::new(false_expr),
                };

                (
                    VarInit::Branch(BranchVarInit {
                        branch_expr,
                        true_scope,
                        false_scope,
                    }),
                    deps,
                )
            }
            Call(expr) => {
                let mut args = Vec::new();
                let mut deps = HashSet::new();

                for arg in &expr.args {
                    let (arg_expr, arg_deps) =
                        self.walk_expr(scope.clone(), arg.clone(), structs, funcs);
                    args.push(Rc::new(arg_expr));
                    deps.extend(arg_deps);
                }

                let func = match &expr.func {
                    func @ Func::Name(_) => func.clone(),
                    Func::Def(func) => Func::Name(funcs.add(func, structs)),
                    Func::Struct(func) => {
                        let ty = structs.walk(&Ty::Struct(func.ty.clone()));

                        Func::Name(NameFunc {
                            name: show_ty(&ty),
                            ty,
                        })
                    }
                };

                (VarInit::Expr(Call(CallExpr { func, args })), deps)
            }
            Field(expr) => {
                let (base_expr, base_deps) =
                    self.walk_expr(scope.clone(), expr.base.clone(), structs, funcs);

                (
                    VarInit::Expr(Field(FieldExpr {
                        base: Rc::new(base_expr),
                        member: expr.member.clone(),
                        ty: structs.walk(&expr.ty),
                    })),
                    base_deps,
                )
            }
            Var(_) => unreachable!(),
            Literal(_) => unreachable!(),
        };

        let var_id = self.next_var_id;
        self.next_var_id.0 += 1;

        eprintln!(
            "declare {:?} = {} in {:?}",
            var_id,
            show_expr(&var_init.expr()),
            Rc::as_ptr(&scope)
        );

        scope.borrow_mut().vars.push((var_id, var_init));

        self.var_ids.insert(expr_ptr(&expr), var_id);
        self.var_infos.insert(
            var_id,
            VarInfo {
                scope,
                deps: deps.clone(),
            },
        );

        deps.insert(var_id);

        (
            Var(VarExpr {
                ident: var_name(var_id),
                ty: expr.ty(),
            }),
            deps,
        )
    }
}
