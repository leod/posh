use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_quote, Error, FnArg, ItemFn, Pat, Result};

pub fn transform(mut item: ItemFn) -> Result<TokenStream2> {
    let mut input_idents = Vec::new();
    let mut input_tys = Vec::new();

    for input in item.sig.inputs.iter_mut() {
        if let FnArg::Typed(input) = input {
            match &*input.pat {
                Pat::Ident(ident) => {
                    input_idents.push(ident.ident.clone());
                    input_tys.push(input.ty.clone());

                    let input_ty = &input.ty;
                    input.ty = parse_quote! { impl ::posh::IntoVal<Val = #input_ty> };
                }
                _ => {
                    return Err(Error::new_spanned(
                        &input.pat,
                        "posh: Only identifiers are allowed as function argument patterns",
                    ));
                }
            }
        }
    }

    let args_ident = quote! { __posh_func_args };

    let func_ident = item.sig.ident.clone();
    let func_body = item.block.clone();

    item.block = parse_quote! {
        {
            const _: fn() = || {
                use ::posh::static_assertions as sa;

                #(
                    sa::assert_impl_all!(#input_tys: ::posh::FuncArgVal);
                )*
            };

            #(
                let #input_idents = ::posh::IntoVal::into_val(#input_idents);
            )*

            let #args_ident = vec![
                #(
                    ::posh::TypedVal::expr(&#input_idents).clone()
                ),*
            ];

            #(
                let #input_idents =
                    <#input_tys as ::posh::TypedVal>::from_ident(
                        ::posh::lang::Ident::new(stringify!(#input_idents)),
                    );
            )*

            ::posh::value::func_def_and_call(
                stringify!(#func_ident),
                vec![
                    #(
                        match ::posh::TypedVal::expr(&#input_idents) {
                            ::posh::lang::Expr::Var(var) => var,
                            _ => unreachable!(),
                        }
                    ),*
                ],
                {
                    //use ::posh::prelude::*;
                    ::posh::IntoVal::into_val(#func_body)
                },
                #args_ident,
            )
        }
    };

    Ok(item.into_token_stream())
}
