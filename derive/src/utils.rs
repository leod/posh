use syn::{parse_quote, Data, Error, Field, Fields, Ident, Path, Result};

pub fn get_struct_fields(ident: &Ident, data: Data) -> Result<Vec<Field>> {
    let fields = if let Data::Struct(data) = data {
        match data.fields {
            Fields::Named(fields) => Ok(fields.named.into_iter().collect()),
            Fields::Unnamed(_) => Err(Error::new_spanned(
                ident,
                "posh derive macros do not support tuple structs",
            )),
            Fields::Unit => Err(Error::new_spanned(
                ident,
                "posh derive macros do not support unit structs",
            )),
        }
    } else {
        Err(Error::new_spanned(
            ident,
            "posh derive macros only support structs",
        ))
    }?;

    Ok(fields)
}

pub trait Derivable {
    fn path() -> Path;
    
}
