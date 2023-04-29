use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

use crate::utils::{validate_generics, StructFields};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    validate_generics(&input.generics)?;

    let ident = &input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = StructFields::new(&input.ident, &input.data)?;
    let field_types = fields.types();

    Ok(quote! {
        unsafe impl #impl_generics ::posh::sl::Const for #ident #ty_generics #where_clause {}

        // Check that all field types implement `Const`.
        const _: fn() = || {
            fn check_field<V: ::posh::sl::Const>() {}

            fn check_struct #impl_generics() #where_clause {
                #(
                    check_field::<#field_types>();
                )*
            }
        };
    })
}
