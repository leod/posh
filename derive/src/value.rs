use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, Ident, Result};

use crate::utils::get_struct_fields;

fn generate_ty_expr(name: &Ident, fields: &Vec<Field>) -> TokenStream {
    let name_str = name.to_string();
    let field_strs: Vec<_> = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect();
    let field_tys: Vec<_> = fields.iter().map(|field| &field.ty).collect();

    quote! {
        ::posh::dag::Ty::Base(
            ::posh::dag::BaseTy::Struct(
                &::posh::dag::StructTy {
                    name: #name_str,
                    fields: &[
                        #(#field_strs, <#field_tys as ::posh::Value>::TY)*
                    ],
                    is_built_in: false,
                }
            )
        )
    }
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let fields = get_struct_fields(&input.ident, input.data)?;
    let ty_expr = generate_ty_expr(&input.ident, &fields);

    Ok(quote! {
        const FOO: ::posh::dag::Ty = #ty_expr;
    })
}
