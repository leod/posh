mod block;
mod r#const;
mod fs_bindings;
mod uniform_bindings;
mod utils;
mod value;
mod varying;
mod vs_bindings;

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

/// Derives `FsBindings` for a struct that is generic in `FsBindingsDom`.
#[proc_macro_derive(FsBindings)]
pub fn derive_fs_bindings(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match fs_bindings::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

/// Derives `UniformBindings` for a struct that is generic in
/// `UniformBindingsDom`.
#[proc_macro_derive(UniformBindings)]
pub fn derive_uniform_data(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match uniform_bindings::derive(input) {
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

/// Derives `VsBindings` for a struct that is generic in `VsBindingsDom`.
#[proc_macro_derive(VsBindings, attributes(vertex))]
pub fn derive_vs_bindings(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match vs_bindings::derive(input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
    .into()
}
