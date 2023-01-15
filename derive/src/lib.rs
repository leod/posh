mod block;
mod resource_interface;
mod utils;
mod value;
mod vertex_interface;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derives `Block` for a struct that is generic in `BlockDomain`.
#[proc_macro_derive(Block)]
pub fn derive_uniform(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match block::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

/// Derives `ResourceInterface` for a struct that is generic in `ResourceDomain`.
#[proc_macro_derive(ResourceInterface)]
pub fn derive_resource_interface(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match resource_interface::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

/// Derives `Value` for a struct.
#[proc_macro_derive(Value)]
pub fn derive_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match value::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

/// Derives `VertexInterface` for a struct that is generic in `VertexDomain`.
#[proc_macro_derive(VertexInterface)]
pub fn derive_vertex_interface(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match vertex_interface::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
