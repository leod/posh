use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Generics, Ident, Result};

use crate::utils::{
    get_domain_param, remove_domain_param, replace_domain_param_bound, SpecializeDomain,
    StructFields,
};

fn generate_helper_fns(
    ident: &Ident,
    generics: &Generics,
    fields: &StructFields,
) -> Result<(Vec<Ident>, TokenStream)> {
    let generics_helper_d = replace_domain_param_bound(
        &get_domain_param(ident, generics)?,
        generics,
        parse_quote!(::posh::macro_internal::UniformDomainMacroHelper),
    )?;
    let (impl_generics_helper_d, _, where_clause) = generics_helper_d.split_for_impl();

    let field_types = fields.types();

    let fn_idents: Vec<_> = fields
        .idents()
        .iter()
        .map(|ident| Ident::new(&format!("_posh_uniform_field_ty_{}", ident), ident.span()))
        .collect();

    let fn_defs = quote! {
        #(
            const fn #fn_idents #impl_generics_helper_d() -> ::posh::dag::Ty
            where #where_clause
            {
                <#field_types as ::posh::sl::Object>::TY
            }
        )*
    };

    Ok((fn_idents, fn_defs))
}

fn generate_struct_ty_expr(
    ident: &Ident,
    generics: &Generics,
    fields: &StructFields,
    helper_fn_idents: &[Ident],
) -> Result<TokenStream> {
    let name_str = ident.to_string();
    let ty_generics_sl = SpecializeDomain::new(parse_quote!(::posh::Sl), &ident, &generics)?;
    let field_strings = fields.strings();

    Ok(quote! {
        ::posh::dag::StructTy {
            name: #name_str,
            fields: &[
                #(
                    (#field_strings, #helper_fn_idents ::#ty_generics_sl())
                ),*
            ],
            is_built_in: false,
        }
    })
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;

    let generics_no_d = remove_domain_param(ident, &input.generics)?;
    let generics_d_type = get_domain_param(ident, &input.generics)?;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (impl_generics_no_d, _, _) = generics_no_d.split_for_impl();
    let ty_generics_gl = SpecializeDomain::new(parse_quote!(::posh::Gl), &ident, &input.generics)?;
    let ty_generics_sl = SpecializeDomain::new(parse_quote!(::posh::Sl), &ident, &input.generics)?;

    let fields = StructFields::new(&input.ident, input.data)?;
    let field_idents = fields.idents();
    let field_types = fields.types();
    let field_strings = fields.strings();

    let (helper_fn_idents, helper_fn_defs) = generate_helper_fns(ident, &input.generics, &fields)?;

    let struct_ty_expr =
        generate_struct_ty_expr(&input.ident, &input.generics, &fields, &helper_fn_idents)?;

    Ok(quote! {
        #helper_fn_defs

        impl #impl_generics ::posh::Uniform<#generics_d_type> for #ident #ty_generics
        #where_clause
        {
            type InGl = #ident #ty_generics_gl;
            type InSl = #ident #ty_generics_sl;
        }

        impl #impl_generics_no_d ::posh::sl::Struct for #ident #ty_generics_sl
        #where_clause
        {
            const STRUCT_TY: ::posh::dag::StructTy = #struct_ty_expr;
        }

        impl #impl_generics_no_d ::posh::sl::Object for #ident #ty_generics_sl
        #where_clause
        {
            const TY: ::posh::dag::Ty =
                ::posh::dag::Ty::Base(::posh::dag::BaseTy::Struct(
                    &<Self as ::posh::sl::Struct>::STRUCT_TY,
                ));

            fn expr(&self) -> std::rc::Rc<::posh::dag::Expr> {
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

            fn check_struct #impl_generics(value: &#ident #ty_generics_sl) #where_clause {
                #(
                    check_field(&value.#field_idents);
                )*
            }
        };

        const _: fn() = || {
            fn check_field<D: ::posh::UniformDomain, T: ::posh::UniformField<D>>() {}

            fn check_struct #impl_generics(value: &#ident #ty_generics) #where_clause {
                #(
                    check_field::<#generics_d_type, #field_types>();
                )*
            }
        };
    })
}
