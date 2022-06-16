mod func;
mod into_value;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemFn};

#[proc_macro_attribute]
pub fn posh(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    match func::transform(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

#[proc_macro_derive(IntoValue, attributes(posh_derive))]
pub fn derive_into_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match into_value::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
