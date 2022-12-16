use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, Ident, Result};

use crate::utils::StructFields;

fn generate_struct_ty_expr(name: &Ident, fields: &StructFields) -> TokenStream {
    let name_str = name.to_string();
    let field_types = fields.types();
    let field_strings = fields.strings();

    quote! {
        &::posh::dag::StructTy {
            name: #name_str,
            fields: &[
                #(
                    (#field_strings, <#field_types as ::posh::sl::Object>::TY)
                ),*
            ],
            is_built_in: false,
        }
    }
}

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    let fields = StructFields::new(&input.ident, input.data)?;
    let field_idents = fields.idents();
    let field_types = fields.types();
    let field_strings = fields.strings();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let struct_ty_expr = generate_struct_ty_expr(&input.ident, &fields);
    let ty_expr = quote! {
        ::posh::dag::Ty::Base(
            ::posh::dag::BaseTy::Struct(
                #struct_ty_expr
            )
        )
    };

    Ok(quote! {
        impl #impl_generics ::posh::sl::Object for #ident #ty_generics #where_clause {
            const TY: ::posh::dag::Ty = #ty_expr;

            fn expr(&self) -> std::rc::Rc<::posh::dag::Expr> {
                ::posh::sl::primitives::simplify_struct_literal(
                    #struct_ty_expr,
                    &[
                        #(
                            self.#field_idents.expr()
                        ),*
                    ]
                )
            }
        }

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

        const _: fn() = || {
            fn check_field<V: ::posh::sl::Value>() {}

            fn check_struct #impl_generics () #where_clause {
                #(
                    check_field::<#field_types>();
                )*
            }
        };
    })
}
