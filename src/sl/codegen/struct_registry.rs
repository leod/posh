use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::sl::dag::{Expr, StructType, Type};

use super::simplified_expr::ExprKey;

type StructId = usize;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct StructKey(*const StructType);

impl<'a> From<&'a Rc<StructType>> for StructKey {
    fn from(value: &'a Rc<StructType>) -> Self {
        StructKey(&**value as *const _)
    }
}

#[derive(Debug, Clone)]
pub struct StructRegistry {
    defs: Vec<Rc<StructType>>,
    ids: HashMap<StructKey, StructId>,
}

impl StructRegistry {
    pub fn new<'a>(roots: &[Rc<Expr>], extra_types: impl Iterator<Item = &'a Type>) -> Self {
        let mut structs = HashMap::new();

        {
            let mut visited = HashSet::new();

            for expr in roots {
                collect_structs_in_expr(expr, &mut visited, &mut structs);
            }
        }

        for ty in extra_types {
            collect_structs_in_type(ty, &mut structs);
        }

        let defs = topological_ordering(structs);
        let ids = defs
            .iter()
            .enumerate()
            .map(|(id, ty)| (StructKey::from(ty), id))
            .collect();

        Self { defs, ids }
    }

    pub fn name(&self, ty: &Rc<StructType>) -> String {
        struct_name(&ty.name, self.ids[&ty.into()])
    }

    pub fn defs(&self) -> impl Iterator<Item = (String, &StructType)> + '_ {
        self.defs
            .iter()
            .enumerate()
            .map(|(id, ty)| (struct_name(&ty.name, id), &**ty))
    }
}

fn struct_name(name: &str, id: StructId) -> String {
    format!("{name}_Posh{id}")
}

fn get_struct_type(ty: &Type) -> Option<&Rc<StructType>> {
    use Type::*;

    match ty {
        BuiltIn(_) => None,
        Array(ty, _) => {
            // This recursion is fine since arrays cannot be nested.
            get_struct_type(ty)
        }
        Struct(ty) => Some(ty),
    }
}

fn collect_structs_in_type(ty: &Type, structs: &mut HashMap<StructKey, Rc<StructType>>) {
    if let Some(ty) = get_struct_type(ty) {
        if structs.insert(ty.into(), ty.clone()).is_some() {
            return;
        }

        for (_, field_ty) in &ty.fields {
            collect_structs_in_type(field_ty, structs);
        }
    }
}

fn collect_structs_in_expr(
    expr: &Rc<Expr>,
    visited: &mut HashSet<ExprKey>,
    structs: &mut HashMap<StructKey, Rc<StructType>>,
) {
    use Expr::*;

    if visited.contains(&expr.into()) {
        return;
    }

    visited.insert(expr.into());

    collect_structs_in_type(&expr.ty(), structs);

    match &**expr {
        Arg { .. } | ScalarLiteral { .. } => (),
        StructLiteral { args, .. } => {
            for arg in args {
                collect_structs_in_expr(arg, visited, structs);
            }
        }
        Binary { left, right, .. } => {
            collect_structs_in_expr(left, visited, structs);
            collect_structs_in_expr(right, visited, structs);
        }
        CallFuncDef { def, args, .. } => {
            collect_structs_in_expr(&def.result, visited, structs);

            for arg in args {
                collect_structs_in_expr(arg, visited, structs);
            }
        }
        CallBuiltIn { args, .. } => {
            for arg in args {
                collect_structs_in_expr(arg, visited, structs);
            }
        }
        Subscript { base, index, .. } => {
            collect_structs_in_expr(base, visited, structs);
            collect_structs_in_expr(index, visited, structs);
        }
        Field { base, .. } => {
            collect_structs_in_expr(base, visited, structs);
        }
        Branch { cond, yes, no, .. } => {
            collect_structs_in_expr(cond, visited, structs);
            collect_structs_in_expr(yes, visited, structs);
            collect_structs_in_expr(no, visited, structs);
        }
    }
}

fn visit(
    ty: &Rc<StructType>,
    permanent_mark: &mut HashSet<StructKey>,
    temporary_mark: &mut HashSet<StructKey>,
    output: &mut Vec<Rc<StructType>>,
) {
    let key: StructKey = ty.into();

    if permanent_mark.contains(&key) {
        return;
    }

    if temporary_mark.contains(&key) {
        panic!("struct definitions contain cycle");
    }

    temporary_mark.insert(key);

    for (_, field_ty) in &ty.fields {
        if let Some(succ) = get_struct_type(field_ty) {
            visit(succ, permanent_mark, temporary_mark, output);
        }
    }

    temporary_mark.remove(&key);
    permanent_mark.insert(key);
    output.push(ty.clone());
}

fn topological_ordering(structs: HashMap<StructKey, Rc<StructType>>) -> Vec<Rc<StructType>> {
    let mut permanent_mark = HashSet::new();
    let mut temporary_mark = HashSet::new();
    let mut output = Vec::new();

    for ty in structs.values() {
        visit(ty, &mut permanent_mark, &mut temporary_mark, &mut output);
    }

    output
}
