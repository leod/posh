use syn::{Data, DataStruct, Error, Field, Fields, Ident, Result};

pub struct StructInput {
    pub fields: Vec<Field>,
}

impl StructInput {
    pub fn new(ident: &Ident, data: Data) -> Result<Self> {
        let fields = if let Data::Struct(data) = data
        {
            match data.fields {
                Fields::Named(fields) => {
                    Ok(fields.named.into_iter().collect())
                } 
                Fields::Unnamed(fields) => {
                }
                Fields::Unit => {

                }
            }
        } else {
            Err(Error::new_spanned(
                ident,
                "posh derive macros only support structs",
            ))
        }?;

        Ok(StructInput { fields })
    }
}
