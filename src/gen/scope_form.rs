use std::collections::{BTreeMap, BTreeSet};

use super::{SimplifiedExpr, VarForm, VarId};

pub type ScopeId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum NodeId {
    Var(VarId),
    Scope(ScopeId),
}

#[derive(Debug, Clone)]
pub enum VarInit<'a> {
    Expr(&'a SimplifiedExpr),
    Branch {
        cond: &'a SimplifiedExpr,
        yes_scope_id: ScopeId,
        no_scope_id: ScopeId,
    },
}

pub struct Scope<'a> {
    pub pred_var_id: Option<VarId>,
    pub exprs: Vec<&'a SimplifiedExpr>,
    pub vars: BTreeMap<VarId, VarInit<'a>>,
}

#[derive(Default)]
struct Graph<'a> {
    scopes: BTreeMap<ScopeId, Scope<'a>>,
    preds: BTreeMap<VarId, BTreeSet<NodeId>>,
    var_inits: BTreeMap<VarId, VarInit<'a>>,
    root_scope_id: ScopeId,
}

impl<'a> Graph<'a> {
    fn add_scope(&mut self, scope: Scope<'a>) -> usize {
        let scope_id = self.scopes.len();

        for succ in &scope.exprs {
            successors(succ, &mut |succ| {
                self.preds
                    .entry(succ)
                    .or_default()
                    .insert(NodeId::Scope(scope_id));
            });
        }

        self.scopes.insert(scope_id, scope);

        scope_id
    }

    fn new(var_form: &'a VarForm) -> Self {
        let mut graph = Self::default();

        for (var_id, var_expr) in var_form.var_exprs().iter().enumerate() {
            use SimplifiedExpr::*;

            //println!("var {} = {} @ {:?}", var_id, var_expr, deps);

            successors(&var_expr, &mut |succ| {
                graph
                    .preds
                    .entry(succ)
                    .or_default()
                    .insert(NodeId::Var(var_id));
            });

            let var_init = match var_expr {
                Branch { cond, yes, no, .. } => {
                    let yes_scope_id = graph.add_scope(Scope {
                        pred_var_id: Some(var_id),
                        exprs: vec![yes],
                        vars: BTreeMap::new(),
                    });

                    let no_scope_id = graph.add_scope(Scope {
                        pred_var_id: Some(var_id),
                        exprs: vec![no],
                        vars: BTreeMap::new(),
                    });

                    VarInit::Branch {
                        cond,
                        yes_scope_id,
                        no_scope_id,
                    }
                }
                Arg { .. }
                | ScalarLiteral { .. }
                | Binary { .. }
                | CallFunc { .. }
                | Field { .. }
                | Var { .. } => VarInit::Expr(var_expr),
            };

            graph.var_inits.insert(var_id, var_init);
        }

        graph.root_scope_id = graph.add_scope(Scope {
            pred_var_id: None,
            exprs: var_form.simplified_roots().to_vec(),
            vars: BTreeMap::new(),
        });

        graph
    }

    fn find_scope_depths(
        &self,
        scope_id: ScopeId,
        depth: usize,
        depths: &mut BTreeMap<ScopeId, usize>,
    ) {
        use VarInit::*;

        depths.insert(scope_id, depth);

        let scope = &self.scopes[&scope_id];

        for (var_id, var_init) in &scope.vars {
            match var_init {
                Expr(_) => (),
                Branch {
                    yes_scope_id,
                    no_scope_id,
                    ..
                } => {
                    self.find_scope_depths(*yes_scope_id, depth + 1, depths);
                    self.find_scope_depths(*no_scope_id, depth + 1, depths);
                }
            }
        }
    }
}

pub struct ScopeForm<'a> {
    var_form: &'a VarForm,
}

impl<'a> ScopeForm<'a> {
    pub fn new(var_form: &VarForm) -> Self {
        let graph = Graph::new(&var_form);

        for (scope_id, scope) in graph.scopes.iter() {
            println!(
                "scope {} in var {:?}: {}",
                scope_id, scope.pred_var_id, scope.exprs[0],
            );
        }

        for (var_id, var_preds) in graph.preds.iter() {
            println!("var {} preds: {:?}", var_id, var_preds);
        }

        let mut scope_depths = BTreeMap::new();
        let scope_depths = graph.find_scope_depths(graph.root_scope_id, 0, &mut scope_depths);

        println!("scope depths: {scope_depths:?}");

        todo!()
    }
}

fn successors(expr: &SimplifiedExpr, f: &mut impl FnMut(VarId)) {
    use SimplifiedExpr::*;

    match expr {
        Branch { cond, .. } => {
            successors(cond, f);
        }
        Arg { .. } | ScalarLiteral { .. } => (),
        Binary { left, right, .. } => {
            successors(left, f);
            successors(right, f);
        }
        CallFunc { args, .. } => {
            for arg in args {
                successors(arg, f);
            }
        }
        Field { base, .. } => {
            successors(base, f);
        }
        Var { id, .. } => {
            f(*id);
        }
    }
}
