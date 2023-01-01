use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Result};

use crate::utils::{get_domain_param, SpecializedTypeGenerics, StructFields};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;

    let generics_d_type = get_domain_param(ident, &input.generics)?;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let ty_generics_gl =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Gl), ident, &input.generics)?;
    let ty_generics_sl =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Sl), ident, &input.generics)?;

    let fields = StructFields::new(&input.ident, &input.data)?;
    let field_idents = fields.idents();
    let field_types = fields.types();
    let field_strings = fields.strings();

    Ok(quote! {
        // Implement `VertexInterface<D>` for the struct.
        impl #impl_generics ::posh::VertexInterface<#generics_d_type>
        for #ident #ty_generics
        #where_clause
        {
            type InGl = #ident #ty_generics_gl;
            type InSl = #ident #ty_generics_sl;

            fn visit(
                &self,
                visitor: &mut impl ::posh::derive_internal::VertexInterfaceVisitor<D>,
            ) {
                #(
                    visitor.accept(
                        #field_strings,
                        ::posh::VertexInputRate::Vertex,
                        &self.#field_idents,
                    );
                )*
            }

            fn shader_input(path: &str) -> Self {
                Self {
                    #(
                        #field_idents:
                            <#field_types as ::posh::VertexInterfaceField<#generics_d_type>>::
                                shader_input(
                                    &::posh::derive_internal::join_ident_path(path, #field_strings),
                                ),
                    )*
                }
            }
        }

        // Check that all field types implement `VertexInterfaceField<D>`.
        const _: fn() = || {
            fn check_field<D: ::posh::VertexDomain, T: ::posh::VertexInterfaceField<D>>() {}

            fn check_struct #impl_generics(value: &#ident #ty_generics) #where_clause {
                #(
                    check_field::<#generics_d_type, #field_types>();
                )*
            }
        };
    })
}
