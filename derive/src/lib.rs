mod to_value;
mod uniform;
mod utils;
mod value;
mod vertex;
mod vertex_interface;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

/// Derives `ToValue` for a struct that is generic in `Domain`.
#[proc_macro_derive(ToValue)]
pub fn derive_to_value(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match to_value::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

/// Derives `Uniform` for a struct that is generic in `Domain`.
#[proc_macro_derive(Uniform)]
pub fn derive_uniform(input: TokenStream) -> TokenStream {
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

/// Derives `Vertex` for a struct that is generic in `Domain`.
#[proc_macro_derive(Vertex)]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match vertex::derive(input) {
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
