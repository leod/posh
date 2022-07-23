mod def;
mod expose;
mod rep;

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

#[proc_macro_derive(Expose, attributes(expose))]
pub fn derive_into_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expose::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

#[proc_macro]
pub fn rep(input: TokenStream) -> TokenStream {
    match rep::transform(input.into()) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
