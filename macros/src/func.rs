use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{parse_quote, Error, ItemFn, Result};

pub fn transform(input: ItemFn) -> Result<TokenStream2> {
    Ok(input.into_token_stream())
}
