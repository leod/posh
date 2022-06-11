use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Fields, Ident, Result};
use uuid::Uuid;

pub fn derive(input: DeriveInput) -> Result<TokenStream2> {
    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => {
            return Err(Error::new_spanned(
                input.ident,
                "derive(Struct) does not support tuple structs, unit structs, enums, or unions",
            ))
        }
    };

    let name = input.ident;
    let vis = input.vis;
    let name_str = name.to_string();

    // FIXME: Using UUIDs in proc macros might break incremental compilation.
    let uuid_str = Uuid::new_v4().to_string();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let field_idents: Vec<_> = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect();
    let field_name_strs: Vec<_> = field_idents.iter().map(|ident| ident.to_string()).collect();
    let field_tys: Vec<_> = fields.iter().map(|field| &field.ty).collect();
    let field_vis: Vec<_> = fields.iter().map(|field| &field.vis).collect();

    let posh_name = Ident::new(&format!("_Posh{}", name), name.span());

    Ok(quote! {
        #[must_use]
        #[derive(Debug, Clone, Copy)]
        #[allow(non_camel_case_types)]
        #vis struct #posh_name #ty_generics #where_clause {
            #(
                #field_vis #field_idents: ::posh::Posh<#field_tys>
            ),*
        }

        impl #impl_generics ::posh::Type for #name #ty_generics #where_clause {
            fn ty() -> ::posh::lang::Ty {
                ::posh::lang::Ty::Struct(<#name as ::posh::Struct>::struct_ty())
            }
        }

        impl #impl_generics ::posh::value::Lift for #name #ty_generics #where_clause {
            type Posh = #posh_name #ty_generics;
        }

        impl #impl_generics ::posh::Value for #posh_name #ty_generics #where_clause {
            type Type = #name #ty_generics;

            fn from_ident(ident: ::posh::lang::Ident) -> Self {
                <Self as ::posh::value::TransparentValue>::from_trace(
                    ::posh::value::Trace::from_ident::<Self>(ident),
                )
            }

            fn expr(&self) -> ::posh::lang::Expr {
                let func = ::posh::lang::Func::Struct(::posh::lang::StructFunc {
                    ty: <#name as ::posh::Struct>::struct_ty(),
                });

                let args = vec![
                    #(
                        ::posh::Value::expr(&self.#field_idents)
                    ),*
                ];

                if let Some(common_base) = ::posh::value::common_field_base(&args) {
                    common_base
                } else {
                    ::posh::lang::Expr::Call(::posh::lang::CallExpr {
                        func,
                        args,
                    })
                }
            }
        }

        impl #impl_generics ::posh::value::TransparentValue for #posh_name #ty_generics
        #where_clause
        {
            fn from_trace(trace: ::posh::value::Trace) -> Self {
                Self {
                    #(
                        #field_idents: ::posh::value::field(trace, #field_name_strs)
                    ),*
                }
            }
        }

        impl #impl_generics ::posh::Struct for #name #ty_generics #where_clause {
            type Posh = #posh_name #ty_generics;

            fn struct_ty() -> ::posh::lang::StructTy {
                let ident = ::posh::lang::Ident {
                    name: #name_str.to_string(),
                    uuid: ::std::str::FromStr::from_str(#uuid_str).unwrap(),
                };

                let fields = vec![
                    #(
                        (#field_name_strs.to_string(), <#field_tys as ::posh::Type>::ty())
                    ),*
                ];

                ::posh::lang::StructTy {
                    ident,
                    fields,
                }
            }
        }

        impl #impl_generics ::posh::IntoPosh for #name #ty_generics #where_clause {
            fn into_posh(self) -> Self::Posh {
                let func = ::posh::lang::Func::Struct(::posh::lang::StructFunc {
                    ty: <#name as ::posh::Struct>::struct_ty(),
                });

                let args = vec![
                    #(
                        ::posh::Value::expr(
                            &<#field_tys as ::posh::IntoPosh>::into_posh(self.#field_idents)
                        )
                    ),*
                ];

                let expr = ::posh::lang::Expr::Call(::posh::lang::CallExpr {
                    func,
                    args,
                });

                <#posh_name as ::posh::value::TransparentValue>::from_expr(expr)
            }
        }
    })
}
