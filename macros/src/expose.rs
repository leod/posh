use std::{collections::HashMap, iter};

use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
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
    fn trait_idents(self) -> Vec<Ident> {
        self.trait_names.into_iter().collect()
    }
}

#[derive(Debug, Clone)]
struct RepTrait {
    name: &'static str,
    deps: &'static [&'static str],
    field_reqs: &'static [fn() -> TokenStream2],
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

    fn field_req_checks(&self, field_tys: &[&Type]) -> TokenStream2 {
        let field_reqs: Vec<_> = self.field_reqs.iter().map(|req| req()).collect();

        TokenStream2::from_iter(field_tys.iter().map(|field_ty| {
            quote_spanned! {field_ty.span()=>
                const _: fn() = || {
                    #(
                        ::posh::static_assertions::assert_impl_all!(
                            ::posh::Rep<#field_ty>: #field_reqs
                        );
                    )*
                };
            }
        }))
    }
}

const REP_TRAITS: &'static [RepTrait] = &[
    RepTrait {
        name: "UniformBlock",
        deps: &["Value"],
        field_reqs: &[|| quote! { ::posh::shader::UniformBlockField }],
    },
    RepTrait {
        name: "Vertex",
        deps: &["Value"],
        field_reqs: &[|| quote! { ::posh::shader::VertexField }],
    },
    RepTrait {
        name: "Interpolants",
        deps: &["Value"],
        field_reqs: &[|| quote! { ::posh::shader::InterpolantsField }],
    },
    RepTrait {
        name: "Fragment",
        deps: &["Value"],
        field_reqs: &[|| quote! { ::posh::shader::FragmentField }],
    },
    RepTrait {
        name: "Value",
        deps: &[],
        field_reqs: &[|| quote! { ::posh::Value }],
    },
    RepTrait {
        name: "Resources",
        deps: &[],
        field_reqs: &[|| quote! { ::posh::shader::Resources }],
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

    let trait_idents = if expose_attrs.is_empty() {
        vec![Ident::new("Value", Span::call_site())]
    } else if expose_attrs.len() == 1 {
        let tokens = expose_attrs.into_iter().next().unwrap().tokens;

        syn::parse2::<ExposeAttr>(tokens)?.trait_idents()
    } else {
        return Err(Error::new_spanned(
            expose_attrs[1].clone(),
            "Can have at most one #[expose(...)] attribute",
        ));
    };

    let mut rep_traits: HashMap<_, _> = trait_idents
        .iter()
        .map(|ident| {
            let rep_trait = get_rep_trait(&ident.to_string()).ok_or_else(|| {
                Error::new_spanned(
                    ident,
                    format!("Unhandled expose trait: {}", ident.to_string()),
                )
            })?;
            Ok((rep_trait.name, rep_trait))
        })
        .collect::<Result<_>>()?;

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

    let posh_name = Ident::new(&format!("_{}PoshRep", name), name.span());

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
    };

    let field_req_checks = rep_traits
        .values()
        .map(|rep_trait| rep_trait.field_req_checks(&field_tys));

    let impl_uniform_block = rep_traits.get("UniformBlock").map(|_| {
        quote! {
            impl #impl_generics ::posh::shader::Resource for #posh_name #ty_generics #where_clause {
                fn stage_arg() -> ::posh::Rep<Self> {
                    // FIXME
                    <Self as ::posh::FuncArg>::from_ident(::posh::lang::Ident::new("input"))
                }
            }

            impl #impl_generics ::posh::shader::UniformBlock
                for #posh_name #ty_generics #where_clause
            {
            }
        }
    });

    let impl_vertex = rep_traits.get("Vertex").map(|_| {
        quote! {
            impl #impl_generics ::posh::shader::Vertex for #posh_name #ty_generics #where_clause
            {
            }
        }
    });

    let impl_interpolants = rep_traits.get("Interpolants").map(|_| {
        quote! {
            impl #impl_generics ::posh::shader::Interpolants
                for #posh_name #ty_generics #where_clause
            {
            }
        }
    });

    let impl_fragment = rep_traits.get("Fragment").map(|_| {
        quote! {
            impl #impl_generics ::posh::shader::Fragment for #posh_name #ty_generics #where_clause
            {
            }
        }
    });

    let impl_value = rep_traits.get("Value").map(|_| {
        quote! {
            impl #impl_generics ::posh::FuncArg for #posh_name #ty_generics #where_clause {
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
                                <::posh::Rep<#field_tys> as ::posh::FuncArg>::ty(),
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
                    let ty = match <Self as ::posh::FuncArg>::ty() {
                        ::posh::lang::Ty::Struct(ty) => ty,
                        _ => unreachable!(),
                    };
                    let struct_func = ::posh::lang::StructFunc { ty };
                    let func = ::posh::lang::Func::Struct(struct_func);

                    let args = vec![
                        #(::posh::FuncArg::expr(&self.#field_idents)),*
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
        }
    });

    let impl_resources = rep_traits.get("Resources").map(|_| {
        quote! {
            impl #impl_generics ::posh::shader::Resources for #posh_name #ty_generics #where_clause
            {
                fn stage_arg() -> ::posh::Rep<Self> {
                    // FIXME
                    ::posh::Rep::<Self> {
                        #(
                            #field_idents: <
                                ::posh::Rep<#field_tys> as ::posh::shader::Resource
                            >::stage_arg()
                        ),*
                    }
                }
            }
        }
    });

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

    Ok(TokenStream2::from_iter(
        iter::once(posh_struct_def)
            .chain(field_req_checks)
            .chain(impl_uniform_block)
            .chain(impl_vertex)
            .chain(impl_interpolants)
            .chain(impl_fragment)
            .chain(impl_value)
            .chain(impl_resources),
    ))
}
