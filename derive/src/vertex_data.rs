use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Result};

use crate::utils::{get_view_param, SpecializedTypeGenerics, StructFields};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;

    let generics_view_type = get_view_param(ident, &input.generics)?;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let ty_generics_logical =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Logical), ident, &input.generics)?;
    let ty_generics_physical =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Physical), ident, &input.generics)?;

    let fields = StructFields::new(&input.ident, &input.data)?;
    let field_idents = fields.idents();
    let field_types = fields.types();
    let field_strings = fields.strings();

    Ok(quote! {
        // Implement `VertexData<V>` for the struct.
        unsafe impl #impl_generics ::posh::VertexData<#generics_view_type>
        for #ident #ty_generics
        #where_clause
        {
            type Logical = #ident #ty_generics_logical;
            type Physical = #ident #ty_generics_physical;

            fn visit<'a>(
                &'a self,
                path: &str,
                visitor: &mut impl ::posh::internal::VertexDataVisitor<'a, #generics_view_type>,
            ) {
                #(
                    visitor.accept(
                        &::posh::internal::join_ident_path(path, #field_strings),
                        ::posh::util::VertexInputRate::Vertex,
                        &self.#field_idents,
                    );
                )*
            }

            fn shader_input(path: &str) -> Self {
                Self {
                    #(
                        #field_idents:
                            <
                                #field_types
                                as
                                ::posh::internal::VertexDataField<#generics_view_type>
                            >::shader_input(
                                &::posh::internal::join_ident_path(path, #field_strings),
                            ),
                    )*
                }
            }
        }

        // Check that all field types implement `VertexDataField<V>`.
        const _: fn() = || {
            fn check_field<V, F>()
            where
                V: ::posh::VertexDataView,
                F: ::posh::internal::VertexDataField<V>,
            {}

            fn check_struct #impl_generics(value: &#ident #ty_generics) #where_clause {
                #(
                    check_field::<#generics_view_type, #field_types>();
                )*
            }
        };
    })
}
