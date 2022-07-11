use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Attribute, Data, DataStruct, DeriveInput, Error, Field, Fields, Ident, Result, Token,
    Type,
};
use uuid::Uuid;

fn struct_fields(ident: &Ident, data: Data) -> Result<Vec<Field>> {
    match data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => Ok(fields.named.into_iter().collect()),
        _ => Err(Error::new_spanned(
            ident.clone(),
            "derive(Expose) does not support tuple structs, unit structs, enums, or unions",
        )),
    }
}

struct ExposeAttr {
    _paren_token: token::Paren,
    trait_names: Punctuated<Ident, Token![,]>,
}

impl Parse for ExposeAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(ExposeAttr {
            _paren_token: parenthesized!(content in input),
            trait_names: content.parse_terminated(Ident::parse)?,
        })
    }
}

impl ExposeAttr {
    fn trait_strings(self) -> Vec<String> {
        self.trait_names
            .into_iter()
            .map(|ident| ident.to_string())
            .collect()
    }
}

#[derive(Debug, Clone)]
struct RepTrait {
    name: &'static str,
    deps: &'static [&'static str],
    field_reqs: &'static [&'static str],
}

impl RepTrait {
    fn add_deps(&self, rep_traits: &mut HashMap<&'static str, RepTrait>) -> bool {
        let mut changed = false;

        for dep in self.deps {
            let is_new = rep_traits
                .insert(dep, get_rep_trait(dep).unwrap())
                .is_none();
            changed = is_new || changed;
        }

        changed
    }

    fn check_field_reqs(&self, field_tys: &[Type]) -> TokenStream2 {
        TokenStream2::from_iter(self.field_reqs.iter().map(|req| {
            quote! {
                const _: fn() = || {
                    use ::posh::static_assertions as sa;

                    #(
                        sa::assert_impl_all!(#field_tys: #req);
                    )*
                };
            }
        }))
    }
}

const REP_TRAITS: &'static [RepTrait] = &[
    RepTrait {
        name: "UniformBlock",
        deps: &["Resource", "Value"],
        field_reqs: &["::posh::shader::UniformBlockField"],
    },
    RepTrait {
        name: "Vertex",
        deps: &["Value"],
        field_reqs: &[], // TODO
    },
    RepTrait {
        name: "VInputs",
        deps: &["Value"],
        field_reqs: &[], // TODO
    },
    RepTrait {
        name: "VOutputs",
        deps: &["Value"],
        field_reqs: &[], // TODO
    },
    RepTrait {
        name: "FOutputs",
        deps: &["Value"],
        field_reqs: &[], // TODO
    },
    RepTrait {
        name: "Value",
        deps: &[],
        field_reqs: &[], // TODO
    },
];

fn get_rep_trait(name: &str) -> Option<RepTrait> {
    REP_TRAITS.iter().find(|rep| rep.name == name).cloned()
}

fn expose_rep_traits(attrs: Vec<Attribute>) -> Result<HashMap<&'static str, RepTrait>> {
    let expose_attrs: Vec<_> = attrs
        .into_iter()
        .filter(|attr| attr.path.is_ident("expose"))
        .collect();

    let trait_strings = if expose_attrs.is_empty() {
        vec!["Value".to_string()]
    } else if expose_attrs.len() == 1 {
        let tokens = expose_attrs.into_iter().next().unwrap().tokens;

        syn::parse2::<ExposeAttr>(tokens)?.trait_strings()
    } else {
        return Err(Error::new_spanned(
            expose_attrs[1].clone(),
            "Can have at most one #[expose(...)] attribute",
        ));
    };

    let mut rep_traits: HashMap<_, _> = trait_strings
        .iter()
        .map(|ident| {
            let rep_trait = get_rep_trait(&ident).unwrap();
            (rep_trait.name, rep_trait)
        })
        .collect();

    loop {
        let mut changed = false;
        let mut new_rep_traits = rep_traits.clone();

        for rep_trait in rep_traits.values() {
            changed = rep_trait.add_deps(&mut new_rep_traits) || changed;
        }

        if !changed {
            break;
        }

        rep_traits = new_rep_traits;
    }

    Ok(rep_traits)
}

pub fn derive(input: DeriveInput) -> Result<TokenStream2> {
    let fields = struct_fields(&input.ident, input.data)?;

    let rep_traits = expose_rep_traits(input.attrs)?;

    let name = input.ident;
    let name_string = name.to_string();
    let vis = input.vis;

    // FIXME: Using UUIDs in proc macros might break incremental compilation.
    let uuid_string = Uuid::new_v4().to_string();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let field_idents: Vec<_> = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect();
    let field_strings: Vec<_> = field_idents.iter().map(|ident| ident.to_string()).collect();
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
                let name = #name_string.to_string();
                let uuid = ::std::str::FromStr::from_str(#uuid_string).unwrap();
                let ident = ::posh::lang::Ident {
                    name,
                    uuid,
                };

                let fields = vec![
                    #(
                        (
                            #field_strings.to_string(),
                            <::posh::Rep<#field_tys> as ::posh::MapToExpr>::ty(),
                        )
                    ),*
                ];

                let struct_ty = ::posh::lang::StructTy {
                    ident,
                    fields,
                };
                ::posh::lang::Ty::Struct(struct_ty)
            }

            fn expr(&self) -> ::posh::lang::Expr {
                let ty = match <Self as ::posh::MapToExpr>::ty() {
                    ::posh::lang::Ty::Struct(ty) => ty,
                    _ => unreachable!(),
                };
                let struct_func = ::posh::lang::StructFunc { ty };
                let func = ::posh::lang::Func::Struct(struct_func);

                let args = vec![
                    #(::posh::MapToExpr::expr(&self.#field_idents)),*
                ];

                if let Some(common_base) = ::posh::expose::common_field_base(&args) {
                    common_base
                } else {
                    let call_expr = ::posh::lang::CallExpr { func, args };
                    ::posh::lang::Expr::Call(call_expr)
                }
            }

            fn from_ident(ident: ::posh::lang::Ident) -> Self {
                let trace = ::posh::expose::Trace::from_ident::<Self>(ident);
                <Self as ::posh::Value>::from_trace(trace)
            }
        }

        impl #impl_generics ::posh::Value for #posh_name #ty_generics #where_clause {
            fn from_trace(trace: ::posh::expose::Trace) -> Self {
                Self {
                    #(#field_idents: ::posh::expose::field(trace, #field_strings)),*
                }
            }
        }

        // TODO: This needs to move to a separate derive(IntoRep).
        /*
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
        */
    };

    Ok(posh_struct_def)
}
