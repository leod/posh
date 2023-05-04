use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Result};

use crate::utils::{
    get_domain_param, remove_domain_param, validate_generics, SpecializedTypeGenerics, StructFields,
};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    validate_generics(&input.generics)?;

    let ident = &input.ident;

    let generics_view_type = get_domain_param(ident, &input.generics)?;
    let generics_init = remove_domain_param(ident, &input.generics)?;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (impl_generics_init, _, where_clause_init) = generics_init.split_for_impl();

    let ty_generics_sl =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Sl), ident, &input.generics)?;
    let ty_generics_gl =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Gl), ident, &input.generics)?;

    let fields = StructFields::new(&input.ident, &input.data)?;
    let field_idents = fields.idents();
    let field_types = fields.types();
    let field_strings = fields.strings();

    Ok(quote! {
        // Implement `UniformBindings<D>` for the struct.
        unsafe impl #impl_generics ::posh::UniformBindings<#generics_view_type>
        for #ident #ty_generics
        #where_clause
        {
            type Sl = #ident #ty_generics_sl;
            type Gl = #ident #ty_generics_gl;

            fn visit<'a>(
                &'a self,
                path: &str,
                visitor: &mut impl ::posh::internal::UniformVisitor<'a, #generics_view_type>,
            ) {
                #(
                    self.#field_idents.visit(
                        &::posh::internal::join_ident_path(path, #field_strings),
                        visitor,
                    );
                )*
            }

            fn shader_input(path: &str) -> Self {
                Self {
                    #(
                        #field_idents:
                            <#field_types as ::posh::UniformBindings<#generics_view_type>>::
                                shader_input(
                                    &::posh::internal::join_ident_path(path, #field_strings),
                                ),
                    )*
                }
            }
        }

        // Implement `UniformNonUnit` for the struct
        impl #impl_generics_init ::posh::UniformNonUnit for #ident #ty_generics_sl
        #where_clause_init
        {}

        // Check that all field types implement `UniformBindings<D>`.
        const _: fn() = || {
            fn check_field<D, U>()
            where
                D: ::posh::UniformBindingsDom,
                U: ::posh::UniformBindings<D>,
            {}

            fn check_struct #impl_generics(value: &#ident #ty_generics) #where_clause {
                #(
                    check_field::<#generics_view_type, #field_types>();
                )*
            }
        };
    })
}
