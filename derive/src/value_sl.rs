use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Result};

use crate::utils::{remove_domain_param, SpecializeDomain, StructFields};

pub fn derive(input: &DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;

    let generics_no_d = remove_domain_param(ident, &input.generics)?;
    let (impl_generics_no_d, _, where_clause) = generics_no_d.split_for_impl();
    let ty_generics_sl = SpecializeDomain::new(parse_quote!(::posh::Sl), ident, &input.generics)?;

    let fields = StructFields::new(&input.ident, &input.data)?;
    let field_idents = fields.idents();
    let field_strings = fields.strings();

    Ok(quote! {
        // Implement `Object` for the struct in `Sl`.
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

        // Implement `Value` for the struct in `Sl`.
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

        // Check that all field types implement `Value`.
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
