use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Fields, Result};

pub fn derive(input: DeriveInput) -> Result<TokenStream2> {
    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => return Err(Error::new_spanned(
            input.ident,
            "derive(Transparent) does not support tuple structs, unit structs, enums, or unions",
        )),
    };

    let name = input.ident;
    let field_tys: Vec<_> = fields.iter().map(|field| &field.ty).collect();

    Ok(quote! {
        impl ::posh::value::Transparent for #name {
            fn transparent() {
                #(
                    <#field_tys as ::posh::value::Transparent>::transparent()
                );*
            }
        }
    })
}
