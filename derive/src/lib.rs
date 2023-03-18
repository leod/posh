mod block;
mod uniform;
mod utils;
mod value;
mod varying;
mod vertex;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derives `Block` for a struct that is generic in `BlockDom`.
#[proc_macro_derive(Block)]
pub fn derive_block(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match block::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

/// Derives `Uniform` for a struct that is generic in `UniformDom`.
#[proc_macro_derive(Uniform)]
pub fn derive_uniform_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match uniform::derive(input) {
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

/// Derives `Varying` for a struct.
#[proc_macro_derive(Varying)]
pub fn derive_varying(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match varying::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

/// Derives `Vertex` for a struct that is generic in `VertexDom`.
#[proc_macro_derive(Vertex)]
pub fn derive_vertex_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match vertex::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
