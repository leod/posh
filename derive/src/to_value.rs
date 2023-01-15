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
        // Implement `Object` for the struct in `Sl`.
        impl #impl_generics_no_d ::posh::sl::Object for #ident #ty_generics_sl
        #where_clause_no_d
        {
            fn ty() -> ::posh::internal::Type {
                ::posh::internal::Type::Base(<Self as ::posh::sl::ValueNonArray>::base_type())
            }

            fn expr(&self) -> ::std::rc::Rc<::posh::internal::Expr> {
                ::posh::internal::simplify_struct_literal(
                    <Self as ::posh::sl::Struct>::struct_type(),
                    &[
                        #(
                            ::posh::sl::Object::expr(&self.#field_idents)
                        ),*
                    ]
                )
            }

            fn from_arg(name: &str) -> Self {
                ::posh::internal::value_arg(name)
            }
        }

        // Implement `Value` for the struct in `Sl`.
        impl #impl_generics_no_d ::posh::sl::Value for #ident #ty_generics_sl
        #where_clause_no_d
        {
            fn from_expr(expr: ::posh::internal::Expr) -> Self {
                let base = ::std::rc::Rc::new(expr);

                Self {
                    #(
                        #field_idents: ::posh::internal::field(
                            base.clone(),
                            #field_strings,
                        )
                    ),*
                }
            }
        }

        // Implement `ValueNonArray` for the struct in `Sl`.
        impl #impl_generics_no_d ::posh::sl::ValueNonArray for #ident #ty_generics_sl
        #where_clause
        {
            fn base_type() -> ::posh::internal::BaseType {
                ::posh::internal::BaseType::Struct(<Self as ::posh::sl::Struct>::struct_type())
            }
        }

        // Implement `Struct` for the struct in `Sl`.
        impl #impl_generics_no_d ::posh::sl::Struct for #ident #ty_generics_sl
        #where_clause
        {
            fn struct_type() -> ::std::rc::Rc<::posh::internal::StructType> {
                ::posh::internal::unique_struct_type::<Self>(
                    || ::posh::internal::StructType {
                        name: #ident_str.to_string(),
                        fields: vec![
                            #(
                                (
                                    #field_strings.to_string(),
                                    <#field_types_sl as ::posh::sl::Object>::ty(),
                                )
                            ),*
                        ],
                    }
                )

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
