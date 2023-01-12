use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Ident, Result};

use crate::utils::{
    remove_domain_param, specialize_field_types, SpecializedTypeGenerics, StructFields,
};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    let visibility = input.vis;

    let as_std140_ident = Ident::new(&format!("PoshInternal{ident}UniformAsStd140"), ident.span());

    let generics_no_d = remove_domain_param(ident, &input.generics)?;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (impl_generics_no_d, ty_generics_no_d, where_clause_no_d) = generics_no_d.split_for_impl();

    let ty_generics_gl =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Gl), ident, &input.generics)?;
    let ty_generics_sl =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Sl), ident, &input.generics)?;

    let fields = StructFields::new(&input.ident, &input.data)?;
    let field_idents = fields.idents();

    let field_types_gl =
        specialize_field_types(parse_quote!(::posh::Gl), ident, &input.generics, &fields)?;
    let field_types_sl =
        specialize_field_types(parse_quote!(::posh::Sl), ident, &input.generics, &fields)?;

    Ok(quote! {
        // Helper type for which we can derive `AsStd140`.
        // FIXME: AFAIK, crevice does not support generic types yet.
        #[doc(hidden)]
        #[derive(::posh::crevice::std140::AsStd140)]
        #visibility struct #as_std140_ident #impl_generics_no_d {
            #(
                #field_idents: #field_types_gl
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

        // Implement `Uniform<Gl>` for the struct in `Gl`.
        impl #impl_generics_no_d ::posh::Uniform<::posh::Gl> for #ident #ty_generics_gl
        #where_clause_no_d
        {
            type InGl = #ident #ty_generics_gl;
            type InSl = #ident #ty_generics_sl;

            fn shader_input(path: &str) -> Self {
                todo!()
            }
        }

        // Implement `Uniform<Sl>` for the struct in `Sl`.
        impl #impl_generics_no_d ::posh::Uniform<::posh::Sl> for #ident #ty_generics_sl
        #where_clause_no_d
        {
            type InGl = #ident #ty_generics_gl;
            type InSl = #ident #ty_generics_sl;

            fn shader_input(path: &str) -> Self {
                ::posh::internal::value_arg(path)
            }
        }

        // Check that all field types in `Gl` implement `Uniform<Gl>`.
        const _: fn() = || {
            fn check_field<T: ::posh::Uniform<::posh::Gl>>() {}

            fn check_struct #impl_generics(value: &#ident #ty_generics) #where_clause {
                #(
                    check_field::<#field_types_gl>();
                )*
            }
        };

        // Check that all field types in `Sl` implement `Uniform<Sl>`.
        const _: fn() = || {
            fn check_field<T: ::posh::Uniform<::posh::Sl>>() {}

            fn check_struct #impl_generics(value: &#ident #ty_generics) #where_clause {
                #(
                    check_field::<#field_types_sl>();
                )*
            }
        };
    })
}
