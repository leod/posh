use std::iter;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse_quote, parse_quote_spanned, spanned::Spanned, Block, Error, Expr, FnArg, Ident, ItemFn,
    Pat, Result, ReturnType, Signature, Stmt, Type,
};

fn inputs(sig: &Signature) -> Result<(Vec<Ident>, Vec<Box<Type>>)> {
    let mut input_idents = Vec::new();
    let mut input_tys = Vec::new();

    for input in sig.inputs.iter() {
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

    Ok((input_idents, input_tys))
}

pub fn transform(mut item: ItemFn) -> Result<TokenStream2> {
    let (input_idents, input_tys) = inputs(&item.sig)?;

    let output_ty = match item.sig.output.clone() {
        ReturnType::Default => {
            return Err(Error::new_spanned(
                &item.sig,
                "posh::def: Function must return a value",
            ));
        }
        ReturnType::Type(_, ty) => ty.clone(),
    };

    let func_ident = item.sig.ident.clone();
    let func_body = item.block.clone();

    let param_idents_var = quote! { _posh_param_idents };

    let param_exprs: Vec<Expr> = input_tys
        .iter()
        .enumerate()
        .map(|(idx, ty)| {
            parse_quote_spanned! {ty.span()=>
                ::posh::lang::FuncParam {
                    ident: #param_idents_var[#idx].clone(),
                    ty: <#ty as ::posh::MapToExpr>::ty(),
                }
            }
        })
        .collect();

    let shadow_param_stmts: Vec<Stmt> = input_tys
        .iter()
        .zip(&input_idents)
        .enumerate()
        .map(|(idx, (ty, ident))| {
            parse_quote_spanned! {ty.span()=>
                let #ident = <#ty as ::posh::MapToExpr>::from_ident(
                    #param_idents_var[#idx].clone(),
                );
            }
        })
        .collect();

    let output_block: Block = parse_quote_spanned! {output_ty.span()=>
        {
            <#output_ty as ::posh::MapToExpr>::expr(&#func_body)
        }
    };

    let arg_exprs: Vec<Expr> = input_tys
        .iter()
        .zip(&input_idents)
        .map(|(ty, ident)| {
            parse_quote_spanned! {ty.span()=>
                <#ty as ::posh::MapToExpr>::expr(&#ident)
            }
        })
        .collect();

    item.block = parse_quote! {
        {
            // Generate Posh identifiers for the function arguments.
            let #param_idents_var = vec![
                #(::posh::lang::Ident::new(stringify!(#input_idents))),*
            ];

            // Return a Posh expression which defines *and* calls the function.
            ::posh::expose::func_def_and_call(
                ::posh::lang::UserDefinedFunc {
                    ident: ::posh::lang::Ident::new(stringify!(#func_ident)),
                    params: vec![#(#param_exprs),*],
                    result: ::std::rc::Rc::new({
                        // Shadow the Rust function arguments with Posh expressions so that
                        // variables in `func_body` refer to the Posh identifiers generated above.
                        #(#shadow_param_stmts)*

                        #[allow(unused_braces)]
                        #output_block
                    })
                },
                vec![#(#arg_exprs),*],
            )
        }
    };

    let output_req_check = quote_spanned! {output_ty.span()=>
        const _: fn() = || {
            ::posh::static_assertions::assert_impl_all!(#output_ty: ::posh::Value);
        };
    };

    Ok(TokenStream2::from_iter(
        iter::once(output_req_check).chain(iter::once(item.into_token_stream())),
    ))
}
