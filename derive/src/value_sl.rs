use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Generics, Ident, Result};

use crate::utils::{
    get_domain_param, remove_domain_param, replace_domain_param_bound, SpecializeDomain,
    StructFields,
};

fn generate_struct_impl(
    ident: &Ident,
    generics: &Generics,
    fields: &StructFields,
) -> Result<TokenStream> {
    let name_str = ident.to_string();

    let generics_no_d = remove_domain_param(ident, generics)?;
    let generics_helper_d = replace_domain_param_bound(
        &get_domain_param(ident, generics)?,
        generics,
        parse_quote!(::posh::macro_internal::UniformDomainHelper),
    )?;

    let (impl_generics_helper_d, _, _) = generics_helper_d.split_for_impl();
    let (impl_generics_no_d, _, where_clause) = generics_no_d.split_for_impl();

    let ty_generics_sl = SpecializeDomain::new(parse_quote!(::posh::Sl), ident, generics)?;

    let field_types = fields.types();
    let field_strings = fields.strings();

    let helper_fn_idents: Vec<_> = fields
        .idents()
        .iter()
        .map(|field_ident| {
            Ident::new(
                &format!("_posh_uniform_field_ty_helper_{ident}_{field_ident}"),
                ident.span(),
            )
        })
        .collect();

    let helper_fn_defs = quote! {
        #(
            #[allow(non_snake_case)]
            const fn #helper_fn_idents #impl_generics_helper_d() -> ::posh::dag::Ty
            where #where_clause
            {
                <#field_types as ::posh::sl::Object>::TY
            }
        )*
    };

    Ok(quote! {
        #helper_fn_defs

        impl #impl_generics_no_d ::posh::sl::Struct for #ident #ty_generics_sl
        #where_clause
        {
            const STRUCT_TY: ::posh::dag::StructTy = ::posh::dag::StructTy {
                name: #name_str,
                fields: &[
                    #(
                        (#field_strings, #helper_fn_idents ::#ty_generics_sl())
                    ),*
                ],
                is_built_in: false,
            };
        }
    })
}

pub fn derive(input: &DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;

    let generics_no_d = remove_domain_param(ident, &input.generics)?;
    let (impl_generics_no_d, _, where_clause) = generics_no_d.split_for_impl();
    let ty_generics_sl = SpecializeDomain::new(parse_quote!(::posh::Sl), ident, &input.generics)?;

    let fields = StructFields::new(&input.ident, &input.data)?;
    let field_idents = fields.idents();
    let field_strings = fields.strings();

    let struct_impl = generate_struct_impl(&input.ident, &input.generics, &fields)?;

    Ok(quote! {
        #struct_impl

        impl #impl_generics_no_d ::posh::sl::Object for #ident #ty_generics_sl
        #where_clause
        {
            const TY: ::posh::dag::Ty = ::posh::dag::Ty::Base(::posh::dag::BaseTy::Struct(
                &<Self as ::posh::sl::Struct>::STRUCT_TY,
            ));

            fn expr(&self) -> ::std::rc::Rc<::posh::dag::Expr> {
                ::posh::sl::primitives::simplify_struct_literal(
                    &<Self as ::posh::sl::Struct>::STRUCT_TY,
                    &[
                        #(
                            self.#field_idents.expr()
                        ),*
                    ]
                )
            }
        }

        impl #impl_generics_no_d ::posh::sl::Value for #ident #ty_generics_sl
        #where_clause
        {
            fn from_expr(expr: ::posh::dag::Expr) -> Self {
                let base = ::std::rc::Rc::new(expr);

                Self {
                    #(
                        #field_idents: ::posh::sl::primitives::field(
                            base.clone(),
                            #field_strings,
                        )
                    ),*
                }
            }
        }

        const _: fn() = || {
            fn check_field<T: ::posh::sl::Value>(_: &T) {}

            fn check_struct #impl_generics_no_d(value: &#ident #ty_generics_sl) #where_clause {
                #(
                    check_field(&value.#field_idents);
                )*
            }
        };
    })
}
