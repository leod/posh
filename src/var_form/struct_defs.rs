use std::collections::HashMap;

use crate::lang::{Ident, NameTy, StructTy, Ty};

fn struct_name(name: &str, num_defs: usize) -> String {
    format!("{}_posh_ty_{}", name, num_defs)
}

#[derive(Debug, Default, Clone)]
pub struct StructDefs {
    defs: Vec<StructTy>,
    map: HashMap<StructTy, NameTy>,
}

impl StructDefs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn defs(&self) -> impl Iterator<Item = &StructTy> {
        self.defs.iter()
    }

    pub fn walk(&mut self, ty: &Ty) -> Ty {
        match ty {
            ty @ Ty::BuiltIn(_) => ty.clone(),
            Ty::Struct(ty) => {
                if let Some(name_ty) = self.map.get(ty) {
                    Ty::Name(name_ty.clone())
                } else {
                    // FIXME: If we ever have deeply nested structs, this recursion might blow up
                    //        exponentially.
                    let fields = ty
                        .fields
                        .iter()
                        .map(|(field_name, field_ty)| (field_name.clone(), self.walk(field_ty)))
                        .collect();

                    let name = struct_name(&ty.ident.name, self.defs.len());

                    let ident = Ident::new(name.clone());
                    self.defs.push(StructTy { ident, fields });

                    let name_ty = NameTy { name };
                    self.map.insert(ty.clone(), name_ty.clone());

                    Ty::Name(name_ty)
                }
            }
            ty @ Ty::Name(_) => ty.clone(),
        }
    }
}
