use syn::{Data, Error, Field, Fields, Ident, Result, Type};

pub struct StructFields {
    fields: Vec<Field>,
}

impl StructFields {
    pub fn new(ident: &Ident, data: Data) -> Result<Self> {
        let data = if let Data::Struct(data) = data {
            Ok(data)
        } else {
            Err(Error::new_spanned(
                ident,
                "posh derive macros only support structs",
            ))
        }?;

        let fields = match data.fields {
            Fields::Named(fields) => Ok(fields.named.into_iter().collect()),
            Fields::Unnamed(_) | Fields::Unit => Err(Error::new_spanned(
                ident,
                "posh derive macros do not support tuple or unit structs",
            )),
        }?;

        Ok(Self { fields })
    }

    pub fn idents(&self) -> Vec<&Ident> {
        self.fields
            .iter()
            .map(|field| field.ident.as_ref().unwrap())
            .collect()
    }

    pub fn types(&self) -> Vec<&Type> {
        self.fields.iter().map(|field| &field.ty).collect()
    }

    pub fn strings(&self) -> Vec<String> {
        self.fields
            .iter()
            .map(|field| field.ident.as_ref().unwrap().to_string())
            .collect()
    }
}
