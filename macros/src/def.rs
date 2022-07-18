use std::iter;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use syn::{parse_quote, spanned::Spanned, Error, FnArg, ItemFn, Pat, Result, ReturnType};

pub fn transform(mut item: ItemFn) -> Result<TokenStream2> {
    let mut input_idents = Vec::new();
    let mut input_tys = Vec::new();

    for input in item.sig.inputs.iter_mut() {
        if let FnArg::Typed(input) = input {
            match &*input.pat {
                Pat::Ident(ident) => {
                    input_idents.push(ident.ident.clone());
                    input_tys.push(input.ty.clone());
                }
                _ => {
                    return Err(Error::new_spanned(
                        &input.pat,
                        "posh::def: Only identifiers are allowed as function argument patterns",
                    ));
                }
            }
        } else {
            // FIXME: What about receivers?
        }
    }

    let args_ident = quote! { __posh_func_args };

    let func_ident = item.sig.ident.clone();
    let func_body = item.block.clone();

    item.block = parse_quote! {
        {
            let #args_ident = vec![
                #(
                    ::posh::MapToExpr::expr(&#input_idents).clone()
                ),*
            ];

            #(
                let #input_idents =
                    <#input_tys as ::posh::MapToExpr>::from_ident(
                        ::posh::lang::Ident::new(stringify!(#input_idents)),
                    );
            )*

            ::posh::expose::func_def_and_call(
                stringify!(#func_ident),
                vec![
                    #(
                        match ::posh::MapToExpr::expr(&#input_idents) {
                            ::posh::lang::Expr::Var(var) => var,
                            _ => unreachable!(),
                        }
                    ),*
                ],
                #func_body,
                #args_ident,
            )
        }
    };

    let arg_req_checks = input_tys.iter().map(|ty| {
        quote_spanned! {ty.span()=>
            const _: fn() = || {
                ::posh::static_assertions::assert_impl_all!(#ty: ::posh::FuncArg);
            };
        }
    });

    let return_ty = match item.sig.output.clone() {
        ReturnType::Default => {
            return Err(Error::new_spanned(
                &item.sig,
                "posh::def: Function must return a value",
            ));
        }
        ReturnType::Type(_, ty) => ty.clone(),
    };

    let result_req_check = quote_spanned! {return_ty.span()=>
        const _: fn() = || {
            ::posh::static_assertions::assert_impl_all!(#return_ty: ::posh::Value);
        };
    };

    Ok(TokenStream2::from_iter(
        arg_req_checks
            .chain(iter::once(result_req_check))
            .chain(iter::once(item.into_token_stream())),
    ))
}
