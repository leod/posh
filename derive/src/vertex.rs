use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Ident, Result};

use crate::utils::{
    get_domain_param, remove_domain_param, specialize_field_types, SpecializedTypeGenerics,
    StructFields,
};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    let visibility = input.vis;

    let to_pod_ident = Ident::new(&format!("PoshInternal{ident}VertexToPod"), ident.span());

    let generics_no_d = remove_domain_param(ident, &input.generics)?;
    let generics_d_type = get_domain_param(ident, &input.generics)?;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (impl_generics_no_d, ty_generics_no_d, where_clause_no_d) = generics_no_d.split_for_impl();

    let ty_generics_gl =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Gl), ident, &input.generics)?;
    let ty_generics_sl =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Sl), ident, &input.generics)?;

    let fields = StructFields::new(&input.ident, &input.data)?;
    let field_idents = fields.idents();
    let field_types = fields.types();
    let field_strings = fields.strings();

    let field_types_gl =
        specialize_field_types(parse_quote!(::posh::Gl), ident, &input.generics, &fields)?;

    Ok(quote! {
        // Helper type for which we can derive `Pod`.
        // FIXME: `Pod` derive does not support generic types and likely never will.
        #[doc(hidden)]
        #[derive(Clone, Copy, ::posh::bytemuck::Zeroable, ::posh::bytemuck::Pod)]
        #[repr(C)]
        #visibility struct #to_pod_ident #impl_generics_no_d {
            #(
                #field_idents: <#field_types_gl as ::posh::internal::ToPod>::Output
            ),*
        }

        // Implement `ToPod` for the struct in `Gl` via the helper type above.
        unsafe impl #impl_generics_no_d ::posh::internal::ToPod for #ident #ty_generics_gl
        #where_clause_no_d
        {
            type Output = #to_pod_ident #ty_generics_no_d;

            fn to_pod(self) -> Self::Output {
                Self::Output {
                    #(
                        #field_idents: self.#field_idents.to_pod()
                    ),*
                }
            }
        }

        // Implement `Vertex<D>` for the struct.
        unsafe impl #impl_generics ::posh::Vertex<#generics_d_type> for #ident #ty_generics
        #where_clause
        {
            type InGl = #ident #ty_generics_gl;
            type InSl = #ident #ty_generics_sl;

            fn attribute_defs(path: &str) -> Vec<::posh::util::VertexAttributeDef> {
                let mut result = Vec::new();

                #(
                    let offset = ::posh::bytemuck::offset_of!(
                        ::posh::bytemuck::Zeroable::zeroed(),
                        #to_pod_ident #ty_generics_no_d,
                        #field_idents
                    );

                    let attrs = <
                        #field_types as ::posh::Vertex<#generics_d_type>
                    >::attribute_defs(&::posh::internal::join_ident_path(path, #field_strings));

                    for attr in attrs {
                        result.push(::posh::util::VertexAttributeDef {
                            offset: attr.offset + offset,
                            ..attr
                        });
                    }
                )*

                result
            }

            fn shader_input(path: &str) -> Self {
                Self {
                    #(
                        #field_idents: <#field_types as ::posh::Vertex<#generics_d_type>>::
                            shader_input(
                                &::posh::internal::join_ident_path(path, #field_strings),
                            ),
                    )*
                }
            }
        }

        // Check that all field types implement `Vertex<D>`.
        const _: fn() = || {
            fn check_field<D: ::posh::Domain, T: ::posh::Vertex<D>>() {}

            fn check_struct #impl_generics(value: &#ident #ty_generics) #where_clause {
                #(
                    check_field::<#generics_d_type, #field_types>();
                )*
            }
        };
    })
}
