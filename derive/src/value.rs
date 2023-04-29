use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Ident, ImplGenerics, Path, Result, Type, WhereClause};

use crate::utils::{validate_generics, StructFields};

pub fn derive_impl(
    ident_str: &str,
    ty: &Path,
    field_idents: &[&Ident],
    field_types: &[&Type],
    (impl_generics, where_clause): (&ImplGenerics, Option<&WhereClause>),
) -> Result<TokenStream> {
    let field_strings: Vec<_> = field_idents.iter().map(|ident| ident.to_string()).collect();

    Ok(quote! {
        // Implement `Struct` for the struct.
        impl #impl_generics ::posh::sl::Struct for #ty #where_clause {
            fn struct_type() -> ::std::rc::Rc<::posh::internal::StructType> {
                ::posh::internal::unique_struct_type::<Self>(
                    || ::posh::internal::StructType  {
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
        impl #impl_generics ::posh::sl::Object for #ty #where_clause {
            fn ty() -> ::posh::internal::Type {
                ::posh::internal::Type::Struct(<Self as ::posh::sl::Struct>::struct_type())
            }

            fn expr(&self) -> ::std::rc::Rc<::posh::internal::Expr> {
                ::posh::internal::simplify_struct_literal(
                    <Self as ::posh::sl::Struct>::struct_type(),
                    &[
                        #(
                            self.#field_idents.expr()
                        ),*
                    ]
                )
            }

            fn from_arg(name: &str) -> Self {
                ::posh::internal::value_arg(name)
            }
        }

        // Implement `Value` for the struct.
        impl #impl_generics ::posh::sl::Value for #ty #where_clause {
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

        // Implement `ValueNonArray` for the struct.
        impl #impl_generics ::posh::sl::ValueNonArray for #ty #where_clause {}

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

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    validate_generics(&input.generics)?;

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = StructFields::new(ident, &input.data)?;

    let value_impl = derive_impl(
        &ident.to_string(),
        &parse_quote!(#ident #ty_generics),
        fields.idents().as_slice(),
        fields.types().as_slice(),
        (&impl_generics, where_clause),
    )?;

    Ok(quote! {
        #value_impl

        // Implement `ToSl` for the struct.
        impl #impl_generics ::posh::sl::ToSl for #ident #where_clause {
            type Output = Self;

            fn to_sl(self) -> Self {
                self
            }
        }
    })
}
