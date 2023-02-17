mod block;
mod uniform_data;
mod utils;
mod value;
mod vertex_data;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derives `Block` for a struct that is generic in `BlockView`.
#[proc_macro_derive(Block)]
pub fn derive_block(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match block::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

/// Derives `UniformData` for a struct that is generic in `UniformDataView`.
#[proc_macro_derive(UniformData)]
pub fn derive_uniform_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match uniform_data::derive(input) {
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

/// Derives `VertexData` for a struct that is generic in `VertexDataView`.
#[proc_macro_derive(VertexData)]
pub fn derive_vertex_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match vertex_data::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
