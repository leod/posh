mod block;
mod r#const;
mod fs_interface;
mod interpolant;
mod uniform;
mod utils;
mod value;
mod vs_interface;

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

/// Derives `Const` for a struct.
#[proc_macro_derive(Const)]
pub fn derive_consts(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match r#const::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

/// Derives `FsInterface` for a struct that is generic in `FsDom`.
#[proc_macro_derive(FsInterface)]
pub fn derive_fs_interface(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match fs_interface::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

/// Derives `Interpolant` for a struct.
#[proc_macro_derive(Interpolant)]
pub fn derive_interpolant(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match interpolant::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

/// Derives `Uniform` for a struct that is generic in `UniformDom`.
#[proc_macro_derive(Uniform)]
pub fn derive_uniform_interface(input: TokenStream) -> TokenStream {
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

/// Derives `VsInterface` for a struct that is generic in `VsDom`.
#[proc_macro_derive(VsInterface, attributes(vertex))]
pub fn derive_vs_interface(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match vs_interface::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
