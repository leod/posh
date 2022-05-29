mod func;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn fush(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    match func::transform(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
