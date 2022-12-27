use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Ident, Result};

use crate::utils::{get_domain_param, remove_domain_param, SpecializeDomain, StructFields};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    let visibility = input.vis;

    let to_pod_ident = Ident::new(&format!("PoshInternal{ident}VertexToPod"), ident.span());
    let gl_field_types_trait = Ident::new(
        &format!("PoshInternal{ident}VertexGlFieldTypes"),
        ident.span(),
    );
    let sl_field_types_trait = Ident::new(
        &format!("PoshInternal{ident}VertexSlFieldTypes"),
        ident.span(),
    );

    let generics_no_d = remove_domain_param(ident, &input.generics)?;
    let generics_d_type = get_domain_param(ident, &input.generics)?;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (impl_generics_no_d, ty_generics_no_d, _) = generics_no_d.split_for_impl();

    let ty_generics_gl = SpecializeDomain::new(parse_quote!(::posh::Gl), ident, &input.generics)?;
    let ty_generics_sl = SpecializeDomain::new(parse_quote!(::posh::Sl), ident, &input.generics)?;

    let fields = StructFields::new(&input.ident, &input.data)?;
    let field_idents = fields.idents();
    let field_types = fields.types();
    let field_strings = fields.strings();

    Ok(quote! {
        // Helper trait for mapping struct field types to `Gl`.
        #[doc(hidden)]
        trait #gl_field_types_trait {
            #(
                #[allow(non_camel_case_types)]
                type #field_idents: ::posh::ToPod;
            )*
        }

        // Implement the helper trait for mapping struct field types to `Gl`.
        impl #impl_generics #gl_field_types_trait for #ident #ty_generics
        #where_clause
        {
            #(
                type #field_idents = <#field_types as ::posh::Vertex<#generics_d_type>>::InGl;
            )*
        }

        // Helper trait for mapping struct field types to `Sl`.
        #[doc(hidden)]
        trait #sl_field_types_trait {
            #(
                #[allow(non_camel_case_types)]
                type #field_idents: ::posh::sl::Object;
            )*
        }

        // Implement the helper trait for mapping struct field types to `Gl`.
        impl #impl_generics #sl_field_types_trait for #ident #ty_generics
        #where_clause
        {
            #(
                type #field_idents = <#field_types as ::posh::Vertex<#generics_d_type>>::InSl;
            )*
        }

        // Helper type for which we can derive `Pod`.
        // FIXME: `Pod` derive does not support generic types and likely never will.
        #[doc(hidden)]
        #[derive(Clone, Copy, ::posh::bytemuck::Zeroable, ::posh::bytemuck::Pod)]
        #[repr(C)]
        #visibility struct #to_pod_ident #impl_generics_no_d {
            #(
                #field_idents: <
                    <#ident #ty_generics_gl as #gl_field_types_trait>::#field_idents
                    as ::posh::ToPod
                >::Output
            ),*
        }

        // Implement `ToPod` for the struct in `Gl` via the helper type above.
        impl #impl_generics_no_d ::posh::ToPod for #ident #ty_generics_gl
        #where_clause
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
        impl #impl_generics ::posh::Vertex<#generics_d_type> for #ident #ty_generics
        #where_clause
        {
            type InGl = #ident #ty_generics_gl;
            type InSl = #ident #ty_generics_sl;

            fn attributes() -> Vec<::posh::VertexAttribute>{
                ::std::vec![
                    #(
                        ::posh::VertexAttribute {
                            name: #field_strings,
                            ty: <
                                <#ident #ty_generics_sl as #sl_field_types_trait>::#field_idents
                                as ::posh::sl::Object
                            >::TYPE,
                            offset: ::posh::bytemuck::offset_of!(
                                ::posh::bytemuck::Zeroable::zeroed(),
                                #to_pod_ident #ty_generics_no_d,
                                #field_idents
                            ),
                        }
                    ),*
                ]
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
