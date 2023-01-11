use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

use crate::utils::{validate_generics, StructFields};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    validate_generics(&input.generics)?;

    let ident = &input.ident;
    let ident_str = ident.to_string();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let fields = StructFields::new(&input.ident, &input.data)?;
    let field_idents = fields.idents();
    let field_types = fields.types();
    let field_strings = fields.strings();

    Ok(quote! {
        // Implement `Struct` for the struct.
        impl #impl_generics ::posh::sl::Struct for #ident #ty_generics #where_clause {
            fn struct_type() -> ::std::rc::Rc<::posh::dag::StructType> {
                ::posh::internal::unique_struct_type::<Self>(
                    || ::posh::dag::StructType  {
                        name: #ident_str.to_string(),
                        fields: vec![
                            #(
                                (
                                    #field_strings.to_string(),
                                    <#field_types as ::posh::sl::Object>::ty(),
                                )
                            ),*
                        ],
                    }
                )
            }
        }

        // Implement `Object` for the struct.
        impl #impl_generics ::posh::sl::Object for #ident #ty_generics #where_clause {
            fn ty() -> ::posh::dag::Type {
                ::posh::dag::Type::Base(::posh::dag::BaseType::Struct(
                    <Self as ::posh::sl::Struct>::struct_type(),
                ))
            }

            fn expr(&self) -> ::std::rc::Rc<::posh::dag::Expr> {
                ::posh::internal::primitives::simplify_struct_literal(
                    <Self as ::posh::sl::Struct>::struct_type(),
                    &[
                        #(
                            self.#field_idents.expr()
                        ),*
                    ]
                )
            }

            fn from_arg(name: &str) -> Self {
                ::posh::internal::primitives::value_arg(name)
            }
        }

        // Implement `Value` for the struct.
        impl #impl_generics ::posh::sl::Value for #ident #ty_generics #where_clause {
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

        // Check that all field types implement `Value`.
        const _: fn() = || {
            fn check_field<V: ::posh::sl::Value>() {}

            fn check_struct #impl_generics() #where_clause {
                #(
                    check_field::<#field_types>();
                )*
            }
        };
    })
}
