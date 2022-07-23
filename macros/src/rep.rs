use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::ToTokens;
use syn::{spanned::Spanned, Error, Result, Type, TypePath};

pub fn rep_name(name: &str) -> String {
    format!("_{}PoshRep", name)
}

fn rep_type(ty: &mut Type) -> Result<()> {
    let span = ty.span();

    match ty {
        Type::Path(TypePath { qself: None, path }) => {
            let mut last_segment = path
                .segments
                .last_mut()
                .ok_or_else(|| Error::new(span, "posh::rep: Empty path not supported"))?;
            last_segment.ident = Ident::new(
                &rep_name(&last_segment.ident.to_string()),
                last_segment.ident.span(),
            );
            Ok(())
        }
        _ => Err(Error::new_spanned(
            &ty,
            "posh::rep: Unsupported kind of type",
        )),
    }
}

pub fn transform(tokens: TokenStream2) -> Result<TokenStream2> {
    let mut ty = syn::parse2::<Type>(tokens)?;
    rep_type(&mut ty)?;

    Ok(ty.into_token_stream())
}
