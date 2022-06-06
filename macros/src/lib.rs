mod func;
mod struct_type;

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

#[proc_macro_derive(Struct)]
pub fn derive_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match struct_type::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
