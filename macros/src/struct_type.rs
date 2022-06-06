use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_quote, Data, DataStruct, DeriveInput, Error, Fields, Ident, Path, Result};
use uuid::Uuid;

pub fn derive(input: DeriveInput) -> Result<TokenStream2> {
    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => return Err(Error::new_spanned(
            input.ident,
            "derive(StructValueType) does not support tuple structs, unit structs, enums, or unions",
        )),
    };

    let type_trait_path: Path = parse_quote!(::posh::value::Type);
    let struct_type_trait_path: Path = parse_quote!(::posh::value::StructType);

    let name = input.ident;
    let vis = input.vis;
    let name_str = name.to_string();
    let uuid_str = Uuid::new_v4().to_string();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let field_idents: Vec<_> = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect();
    let field_name_strs: Vec<_> = field_idents.iter().map(|ident| ident.to_string()).collect();
    let field_tys: Vec<_> = fields.iter().map(|field| &field.ty).collect();
    let field_vis: Vec<_> = fields.iter().map(|field| &field.vis).collect();

    let posh_name = Ident::new(&format!("__posh_{}", name), name.span());

    Ok(quote! {
        #[must_use]
        #[derive(Debug, Clone, Copy)]
        #[allow(non_camel_case_types)]
        #vis struct #posh_name {
            trace: ::posh::value::Trace,
            #(
                #field_vis #field_idents: ::posh::Posh<#field_tys>
            ),*
        }

        impl ::posh::Value for #posh_name {
            type Type = #name;

            fn from_trace(trace: ::posh::value::Trace) -> Self {
                Self {
                    trace,
                    #(
                        #field_idents: ::posh::value::field(trace, #field_name_strs)
                    ),*
                }
            }

            fn trace(&self) -> ::posh::value::Trace {
                self.trace
            }
        }

        impl #impl_generics #type_trait_path for #name #ty_generics #where_clause {
            type Value = #posh_name;

            fn ty() -> ::posh::lang::Ty {
                ::posh::lang::Ty::Struct(<#name as ::posh::StructType>::struct_ty())
            }
        }

        impl #impl_generics #struct_type_trait_path for #name #ty_generics #where_clause {
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

        impl ::posh::IntoValue for #name {
            type Value = #posh_name;

            fn into_value(self) -> Self::Value {
                let func = ::posh::lang::Func::Struct(::posh::lang::StructFunc {
                    ty: <#name as ::posh::StructType>::struct_ty(),
                });

                let args = vec![
                    #(
                        <#field_tys as ::posh::IntoValue>::into_value(self.#field_idents).expr()
                    ),*
                ];

                let expr = ::posh::lang::Expr::Call(::posh::lang::CallExpr {
                    func,
                    args,
                });

                <#posh_name as ::posh::Value>::from_expr(expr)
            }
        }
    })
}
