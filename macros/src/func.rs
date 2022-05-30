use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_quote, Error, FnArg, ItemFn, Pat, Result};

pub fn transform(mut item: ItemFn) -> Result<TokenStream2> {
    let mut input_idents = Vec::new();

    for input in item.sig.inputs.iter_mut() {
        if let FnArg::Typed(input) = input {
            let input_ty = &input.ty;
            input.ty = parse_quote! { impl Into<#input_ty> };

            match &mut *input.pat {
                Pat::Ident(ident) => {
                    input_idents.push(ident.ident.clone());
                }
                _ => {
                    return Err(Error::new_spanned(
                        &input.pat,
                        "fsl: Only identifiers are allowed as function argument patterns",
                    ));
                }
            }
        }
    }

    let args_ident = quote! { __fsl_func_args };

    let func_ident = item.sig.ident.clone();
    let func_body = item.block.clone();

    item.block = parse_quote! {
        {
            use ::fsl::value::Value as _;

            #(
                let #input_idents = #input_idents.into();
            )*

            let #args_ident = vec![
                #(
                    #input_idents.expr().clone()
                ),*
            ];

            #(
                let #input_idents = #input_idents.map_expr(|_| {
                    ::fsl::lang::Expr::Var(::fsl::lang::ExprVar {
                        var: ::fsl::lang::Var {
                            ident: ::fsl::lang::Ident::new(stringify!(#input_idents)),
                            ty: #input_idents.ty(),
                        },
                        init: None,
                    })
                });
            )*

            ::fsl::value::func_call(
                stringify!(#func_ident),
                vec![
                    #(
                        ::fsl::lang::Var {
                            ident: ::fsl::lang::Ident::new(stringify!(#input_idents)),
                            ty: #input_idents.ty(),
                        }
                    ),*
                ],
                {
                    #func_body
                },
                #args_ident,
            )
        }
    };

    Ok(item.into_token_stream())
}
