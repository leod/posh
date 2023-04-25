use std::{
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

use crate::sl::dag::{ArrayType, Expr, StructType, Type};

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
    ids: BTreeMap<StructKey, StructId>,
}

impl StructRegistry {
    pub fn new<'a>(roots: &[Rc<Expr>], extra_types: impl Iterator<Item = &'a Type>) -> Self {
        let mut structs = BTreeMap::new();
        let mut structs_insertion_order = Vec::new();

        // The names of structs that occur in uniform block declarations must
        // match between shader stages. Thus, we need to process `extra_types`
        // first, so that their structs get the same names in all shader stages.
        for ty in extra_types {
            collect_structs_in_type(ty, &mut structs, &mut structs_insertion_order);
        }

        {
            let mut visited = BTreeSet::new();

            for expr in roots {
                collect_structs_in_expr(
                    expr,
                    &mut visited,
                    &mut structs,
                    &mut structs_insertion_order,
                );
            }
        }

        let defs = topological_ordering(&structs_insertion_order);
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
        Array(ArrayType { ty, .. }) => {
            // This recursion is fine since arrays cannot be nested.
            get_struct_type(ty)
        }
        Struct(ty) => Some(ty),
    }
}

fn collect_structs_in_type(
    ty: &Type,
    structs: &mut BTreeMap<StructKey, Rc<StructType>>,
    structs_insertion_order: &mut Vec<Rc<StructType>>,
) {
    if let Some(ty) = get_struct_type(ty) {
        if structs.insert(ty.into(), ty.clone()).is_some() {
            return;
        }

        structs_insertion_order.push(ty.clone());

        for (_, field_ty) in &ty.fields {
            collect_structs_in_type(field_ty, structs, structs_insertion_order);
        }
    }
}

fn collect_structs_in_expr(
    expr: &Rc<Expr>,
    visited: &mut BTreeSet<ExprKey>,
    structs: &mut BTreeMap<StructKey, Rc<StructType>>,
    structs_insertion_order: &mut Vec<Rc<StructType>>,
) {
    if visited.contains(&expr.into()) {
        return;
    }

    visited.insert(expr.into());

    collect_structs_in_type(&expr.ty(), structs, structs_insertion_order);

    expr.successors(|succ| {
        collect_structs_in_expr(succ, visited, structs, structs_insertion_order)
    });
}

fn visit(
    ty: &Rc<StructType>,
    permanent_mark: &mut BTreeSet<StructKey>,
    temporary_mark: &mut BTreeSet<StructKey>,
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

fn topological_ordering(structs: &[Rc<StructType>]) -> Vec<Rc<StructType>> {
    let mut permanent_mark = BTreeSet::new();
    let mut temporary_mark = BTreeSet::new();
    let mut output = Vec::new();

    for ty in structs {
        visit(ty, &mut permanent_mark, &mut temporary_mark, &mut output);
    }

    output
}
