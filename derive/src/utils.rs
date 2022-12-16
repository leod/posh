use syn::{Data, Error, Field, Fields, Ident, Result};

pub fn get_struct_fields(ident: &Ident, data: Data) -> Result<Vec<Field>> {
    let data = if let Data::Struct(data) = data {
        Ok(data)
    } else {
        Err(Error::new_spanned(
            ident,
            "posh derive macros only support structs",
        ))
    }?;

    match data.fields {
        Fields::Named(fields) => Ok(fields.named.into_iter().collect()),
        Fields::Unnamed(_) | Fields::Unit => Err(Error::new_spanned(
            ident,
            "posh derive macros do not support tuple or unit structs",
        )),
    }
}
