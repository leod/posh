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
            const STRUCT_TY: ::posh::dag::StructTy = ::posh::dag::StructTy {
                name: #ident_str,
                fields: &[
                    #(
                        (#field_strings, <#field_types as ::posh::sl::Object>::TY)
                    ),*
                ],
                is_built_in: false,
            };
        }

        // Implement `Object` for the struct.
        impl #impl_generics ::posh::sl::Object for #ident #ty_generics #where_clause {
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

        // Implement `Value` for the struct.
        impl #impl_generics ::posh::sl::Value for #ident #ty_generics #where_clause {
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
