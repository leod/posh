use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::dag::{BaseType, Expr, StructType, Type};

use super::ExprKey;

type StructId = usize;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct StructKey(*const StructType);

impl<'a> From<&'static StructType> for StructKey {
    fn from(value: &'static StructType) -> Self {
        StructKey(value as *const _)
    }
}

#[derive(Debug, Clone)]
pub struct StructRegistry {
    defs: Vec<&'static StructType>,
    ids: HashMap<StructKey, StructId>,
}

impl StructRegistry {
    pub fn new<'a>(roots: &[Rc<Expr>]) -> Self {
        let mut structs = HashMap::new();

        {
            let mut visited = HashSet::new();

            for expr in roots {
                collect_structs(expr, &mut visited, &mut structs);
            }
        }

        let defs = topological_ordering(structs);
        let ids = defs
            .iter()
            .enumerate()
            .map(|(id, &ty)| (StructKey::from(ty), id))
            .collect();

        Self { defs, ids }
    }

    pub fn name(&self, ty: &'static StructType) -> String {
        struct_name(ty.name, self.ids[&ty.into()])
    }

    pub fn defs(&self) -> impl Iterator<Item = (String, &'static StructType)> + '_ {
        self.defs
            .iter()
            .enumerate()
            .map(|(id, ty)| (struct_name(ty.name, id), *ty))
    }
}

fn struct_name(name: &'static str, id: StructId) -> String {
    format!("{name}_Posh{id}")
}

fn get_struct_type(ty: &Type) -> Option<&'static StructType> {
    use Type::*;

    match ty {
        Array(BaseType::Struct(ty), _) | Base(BaseType::Struct(ty)) if !ty.is_built_in => Some(ty),
        Array(_, _) | Base(_) => None,
    }
}

fn collect_structs(
    expr: &Rc<Expr>,
    visited: &mut HashSet<ExprKey>,
    structs: &mut HashMap<StructKey, &'static StructType>,
) {
    use Expr::*;

    if visited.contains(&expr.into()) {
        return;
    }

    visited.insert(expr.into());

    if let Some(ty) = get_struct_type(&expr.ty()) {
        structs.insert(ty.into(), ty);
    }

    match &**expr {
        Arg { .. } | ScalarLiteral { .. } => (),
        StructLiteral { args, .. } => {
            for arg in args {
                collect_structs(arg, visited, structs);
            }
        }
        Binary { left, right, .. } => {
            collect_structs(left, visited, structs);
            collect_structs(right, visited, structs);
        }
        CallFuncDef { def, args, .. } => {
            collect_structs(&def.result, visited, structs);

            for arg in args {
                collect_structs(arg, visited, structs);
            }
        }
        CallBuiltIn { args, .. } => {
            for arg in args {
                collect_structs(arg, visited, structs);
            }
        }
        Field { base, .. } => {
            collect_structs(base, visited, structs);
        }
        Branch { cond, yes, no, .. } => {
            collect_structs(cond, visited, structs);
            collect_structs(yes, visited, structs);
            collect_structs(no, visited, structs);
        }
    }
}

fn visit(
    ty: &'static StructType,
    permanent_mark: &mut HashSet<StructKey>,
    temporary_mark: &mut HashSet<StructKey>,
    output: &mut Vec<&'static StructType>,
) {
    let key: StructKey = ty.into();

    if permanent_mark.contains(&key) {
        return;
    }

    if temporary_mark.contains(&key) {
        panic!("Struct definitions contain cycle");
    }

    temporary_mark.insert(key);

    for (_, field_ty) in ty.fields {
        if let Some(succ) = get_struct_type(field_ty) {
            visit(succ, permanent_mark, temporary_mark, output);
        }
    }

    temporary_mark.remove(&key);
    permanent_mark.insert(key);
    output.push(ty);
}

fn topological_ordering(
    structs: HashMap<StructKey, &'static StructType>,
) -> Vec<&'static StructType> {
    let mut permanent_mark = HashSet::new();
    let mut temporary_mark = HashSet::new();
    let mut output = Vec::new();

    for ty in structs.values() {
        visit(ty, &mut permanent_mark, &mut temporary_mark, &mut output);
    }

    output
}
