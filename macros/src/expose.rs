use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Fields, Ident, Result};
use uuid::Uuid;

pub fn derive(input: DeriveInput) -> Result<TokenStream2> {
    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => Ok(&fields.named),
        _ => Err(Error::new_spanned(
            input.ident.clone(),
            "derive(Expose) does not support tuple structs, unit structs, enums, or unions",
        )),
    }?;

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

    let posh_name = Ident::new(&format!("_PoshRepr{}", name), name.span());

    Ok(quote! {
        #[must_use]
        #[derive(Debug, Clone, Copy)]
        #[allow(non_camel_case_types)]
        #vis struct #posh_name #ty_generics #where_clause {
            #(
                #field_vis #field_idents: ::posh::Rep<#field_tys>
            ),*
        }

        impl #impl_generics ::posh::Expose for #name #ty_generics #where_clause {
            type Rep = #posh_name #ty_generics;
        }

        impl #impl_generics ::posh::Expose for #posh_name #ty_generics #where_clause {
            type Rep = Self;
        }

        impl #impl_generics ::posh::Representant for #posh_name #ty_generics #where_clause {
        }

        impl #impl_generics ::posh::Value for #posh_name #ty_generics #where_clause {
            fn ty() -> ::posh::lang::Ty {
                let ident = ::posh::lang::Ident {
                    name: #name_str.to_string(),
                    uuid: ::std::str::FromStr::from_str(#uuid_str).unwrap(),
                };

                let fields = vec![
                    #(
                        (
                            #field_name_strs.to_string(),
                            <::posh::Rep<#field_tys> as ::posh::Value>::ty(),
                        )
                    ),*
                ];

                ::posh::lang::Ty::Struct(
                    ::posh::lang::StructTy {
                        ident,
                        fields,
                    },
                )
            }

            fn expr(&self) -> ::posh::lang::Expr {
                let func = ::posh::lang::Func::Struct(::posh::lang::StructFunc {
                    ty: match <Self as ::posh::Value>::ty() {
                        ::posh::lang::Ty::Struct(ty) => ty,
                        _ => unreachable!(),
                    },
                });

                let args = vec![
                    #(
                        ::posh::Value::expr(&self.#field_idents)
                    ),*
                ];

                if let Some(common_base) = ::posh::expose::common_field_base(&args) {
                    common_base
                } else {
                    ::posh::lang::Expr::Call(::posh::lang::CallExpr {
                        func,
                        args,
                    })
                }
            }

            fn from_ident(ident: ::posh::lang::Ident) -> Self {
                <Self as ::posh::Transparent>::from_trace(
                    ::posh::expose::Trace::from_ident::<Self>(ident),
                )
            }
        }

        impl #impl_generics ::posh::Transparent for #posh_name #ty_generics
        #where_clause
        {
            fn from_trace(trace: ::posh::expose::Trace) -> Self {
                Self {
                    #(
                        #field_idents: ::posh::expose::field(trace, #field_name_strs)
                    ),*
                }
            }
        }

        impl #impl_generics ::posh::IntoRep for #name #ty_generics #where_clause {
            fn into_rep(self) -> Self::Rep {
                #posh_name {
                    #(
                        #field_idents: <#field_tys as ::posh::IntoRep>::into_rep(
                            self.#field_idents,
                        )
                    ),*
                }
            }
        }
    })
}
