mod def;
mod expose;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemFn};

#[proc_macro_attribute]
pub fn def(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    match def::transform(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

#[proc_macro_derive(Expose, attributes(expose_derive))]
pub fn derive_into_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expose::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
