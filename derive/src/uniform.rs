use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Ident, Result};

use crate::utils::{get_domain_param, remove_domain_param, SpecializeDomain, StructFields};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    let visibility = input.vis;

    let as_std140_ident = Ident::new(&format!("PoshInternal{ident}UniformAsStd140"), ident.span());
    let gl_field_types_trait = Ident::new(
        &format!("PoshInternal{ident}UniformGlFieldTypes"),
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

    Ok(quote! {
        // Helper trait for mapping struct field types to `Gl`.
        #[doc(hidden)]
        trait #gl_field_types_trait {
            #(
                #[allow(non_camel_case_types)]
                type #field_idents: ::posh::crevice::std140::AsStd140;
            )*
        }

        // Implement the helper trait for mapping struct field types to `Gl`.
        impl #impl_generics #gl_field_types_trait for #ident #ty_generics
        #where_clause
        {
            #(
                type #field_idents = <#field_types as ::posh::Uniform<#generics_d_type>>::InGl;
            )*
        }

        // Helper type for which we can derive `AsStd140`.
        // FIXME: AFAIK, crevice does not support generic types yet.
        #[doc(hidden)]
        #[derive(::posh::crevice::std140::AsStd140)]
        #visibility struct #as_std140_ident #impl_generics_no_d {
            #(
                #field_idents: <#ident #ty_generics_gl as #gl_field_types_trait>::#field_idents
            ),*
        }

        // Implement `AsStd140` for the struct in `Gl` via the helper type above.
        impl #impl_generics_no_d ::posh::crevice::std140::AsStd140 for #ident #ty_generics_gl
        #where_clause
        {
            type Output = <
                #as_std140_ident #ty_generics_no_d as ::posh::crevice::std140::AsStd140
            >::Output;

            fn as_std140(&self) -> Self::Output {
                #as_std140_ident {
                    #(
                        #field_idents: self.#field_idents.clone()
                    ),*
                }
                .as_std140()
            }

            fn from_std140(val: Self::Output) -> Self {
                Self {
                    #(
                        #field_idents: ::posh::crevice::std140::AsStd140::from_std140(
                            val.#field_idents,
                        )
                    ),*
                }
            }
        }

        // Implement `Uniform<D>` for the struct.
        impl #impl_generics ::posh::Uniform<#generics_d_type> for #ident #ty_generics
        #where_clause
        {
            type InGl = #ident #ty_generics_gl;
            type InSl = #ident #ty_generics_sl;
        }

        // Check that all field types implement `Uniform<D>`.
        const _: fn() = || {
            fn check_field<D: ::posh::Domain, T: ::posh::Uniform<D>>() {}

            fn check_struct #impl_generics(value: &#ident #ty_generics) #where_clause {
                #(
                    check_field::<#generics_d_type, #field_types>();
                )*
            }
        };
    })
}
