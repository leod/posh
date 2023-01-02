use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Result};

use crate::utils::{
    remove_domain_param, specialize_field_types, SpecializedTypeGenerics, StructFields,
};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    let ident_str = ident.to_string();

    let generics_no_d = remove_domain_param(ident, &input.generics)?;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (impl_generics_no_d, _, where_clause_no_d) = generics_no_d.split_for_impl();

    let ty_generics_sl =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Sl), ident, &input.generics)?;

    let fields = StructFields::new(&input.ident, &input.data)?;
    let field_idents = fields.idents();
    let field_strings = fields.strings();

    let field_types_sl =
        specialize_field_types(parse_quote!(::posh::Sl), ident, &input.generics, &fields)?;

    Ok(quote! {
        // Implement `Struct` for the struct in `Sl`.
        impl #impl_generics_no_d ::posh::sl::Struct for #ident #ty_generics_sl
        #where_clause
        {
            const STRUCT_TYPE: ::posh::dag::StructType = ::posh::dag::StructType {
                name: #ident_str,
                fields: &[
                    #(
                        (
                            #field_strings,
                            <#field_types_sl as ::posh::sl::Object>::TYPE,
                        )
                    ),*
                ],
                is_built_in: false,
            };
        }

        // Implement `Object` for the struct in `Sl`.
        impl #impl_generics_no_d ::posh::sl::Object for #ident #ty_generics_sl
        #where_clause_no_d
        {
            const TYPE: ::posh::dag::Type = ::posh::dag::Type::Base(::posh::dag::BaseType::Struct(
                &<Self as ::posh::sl::Struct>::STRUCT_TYPE,
            ));

            fn expr(&self) -> ::std::rc::Rc<::posh::dag::Expr> {
                ::posh::internal::primitives::simplify_struct_literal(
                    &<Self as ::posh::sl::Struct>::STRUCT_TYPE,
                    &[
                        #(
                            ::posh::sl::Object::expr(&self.#field_idents)
                        ),*
                    ]
                )
            }

            fn from_arg(name: &str) -> Self {
                ::posh::internal::primitives::value_arg(name)
            }
        }

        // Implement `Value` for the struct in `Sl`.
        impl #impl_generics_no_d ::posh::sl::Value for #ident #ty_generics_sl
        #where_clause_no_d
        {
            fn from_expr(expr: ::posh::dag::Expr) -> Self {
                let base = ::std::rc::Rc::new(expr);

                Self {
                    #(
                        #field_idents: ::posh::internal::primitives::field(
                            base.clone(),
                            #field_strings,
                        )
                    ),*
                }
            }
        }

        // Implement `ToValue` for the struct in all domains.
        impl #impl_generics ::posh::sl::ToValue for #ident #ty_generics
        #where_clause
        {
            type Output = #ident #ty_generics_sl;

            fn to_value(self) -> Self::Output {
                Self::Output {
                    #(
                        #field_idents: ::posh::sl::ToValue::to_value(self.#field_idents)
                    ),*
                }
            }
        }
    })
}
