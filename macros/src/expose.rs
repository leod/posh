use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Data, DataStruct, DeriveInput, Error, Fields, Ident, Result, Token,
};
use uuid::Uuid;

struct ExposeAttr {
    paren_token: token::Paren,
    trait_names: Punctuated<Ident, Token![,]>,
}

impl Parse for ExposeAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(ExposeAttr {
            paren_token: parenthesized!(content in input),
            trait_names: content.parse_terminated(Ident::parse)?,
        })
    }
}

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

    let expose_attrs: Vec<_> = input
        .attrs
        .into_iter()
        .filter(|attr| attr.path.is_ident("expose"))
        .collect();

    let expose_attr: Option<ExposeAttr> = if expose_attrs.is_empty() {
        None
    } else if expose_attrs.len() == 1 {
        Some(syn::parse2(
            expose_attrs.into_iter().next().unwrap().tokens,
        )?)
    } else {
        return Err(Error::new_spanned(
            expose_attrs[1].clone(),
            "Can have at most one #[expose(...)] attribute",
        ));
    };

    if let Some(attr) = expose_attr.as_ref() {
        for trait_name in attr.trait_names.iter() {
            panic!("{}", trait_name);
        }
    }

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

    let posh_struct_def = quote! {
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

        impl #impl_generics ::posh::Representative for #posh_name #ty_generics #where_clause {}

        impl #impl_generics ::posh::MapToExpr for #posh_name #ty_generics #where_clause {
            fn ty() -> ::posh::lang::Ty {
                let ident = ::posh::lang::Ident {
                    name: #name_str.to_string(),
                    uuid: ::std::str::FromStr::from_str(#uuid_str).unwrap(),
                };

                let fields = vec![
                    #(
                        (
                            #field_name_strs.to_string(),
                            <::posh::Rep<#field_tys> as ::posh::MapToExpr>::ty(),
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
                    ty: match <Self as ::posh::MapToExpr>::ty() {
                        ::posh::lang::Ty::Struct(ty) => ty,
                        _ => unreachable!(),
                    },
                });

                let args = vec![
                    #(
                        ::posh::MapToExpr::expr(&self.#field_idents)
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
                <Self as ::posh::Value>::from_trace(
                    ::posh::expose::Trace::from_ident::<Self>(ident),
                )
            }
        }

        impl #impl_generics ::posh::Value for #posh_name #ty_generics
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

        // TODO: This needs to move to a separate derive(IntoRep).
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
    };

    Ok(posh_struct_def)
}
