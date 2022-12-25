use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Data, Error, Field, Fields, GenericParam, Generics, Ident, Result, Token, Type};

pub struct StructFields {
    fields: Vec<Field>,
}

impl StructFields {
    pub fn new(ident: &Ident, data: &Data) -> Result<Self> {
        let data = if let Data::Struct(data) = data {
            Ok(data)
        } else {
            Err(Error::new_spanned(
                ident,
                "posh derive macros only support structs",
            ))
        }?;

        let fields = match &data.fields {
            Fields::Named(fields) => Ok(fields.named.iter().cloned().collect()),
            Fields::Unnamed(_) | Fields::Unit => Err(Error::new_spanned(
                ident,
                "posh derive macros do not support tuple or unit structs",
            )),
        }?;

        Ok(Self { fields })
    }

    pub fn idents(&self) -> Vec<&Ident> {
        self.fields
            .iter()
            .map(|field| field.ident.as_ref().unwrap())
            .collect()
    }

    pub fn types(&self) -> Vec<&Type> {
        self.fields.iter().map(|field| &field.ty).collect()
    }

    pub fn strings(&self) -> Vec<String> {
        self.fields
            .iter()
            .map(|field| field.ident.as_ref().unwrap().to_string())
            .collect()
    }
}

pub struct SpecializeDomain {
    first_ty: Type,
    params: Vec<GenericParam>,
}

impl SpecializeDomain {
    pub fn new(first_ty: Type, ident: &Ident, generics: &Generics) -> Result<Self> {
        if generics.params.is_empty() {
            return Err(Error::new_spanned(
                ident,
                "posh expects type to be generic in domain",
            ));
        }

        Ok(Self {
            first_ty,
            params: generics.params.iter().skip(1).cloned().collect(),
        })
    }
}

impl ToTokens for SpecializeDomain {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        <Token![<]>::default().to_tokens(tokens);

        self.first_ty.to_tokens(tokens);
        <Token![,]>::default().to_tokens(tokens);

        for param in &self.params {
            match param {
                GenericParam::Lifetime(_) => {
                    panic!("Internal error: posh does not support lifetimes");
                }
                GenericParam::Type(param) => {
                    // Leave off the type parameter defaults.
                    param.ident.to_tokens(tokens);
                }
                GenericParam::Const(param) => {
                    // Leave off the const parameter defaults.
                    param.ident.to_tokens(tokens);
                }
            }

            <Token![,]>::default().to_tokens(tokens);
        }
        <Token![>]>::default().to_tokens(tokens);
    }
}

pub fn validate_generics(generics: &Generics) -> Result<()> {
    for param in generics.params.iter() {
        if let GenericParam::Lifetime(param) = param {
            return Err(Error::new_spanned(
                param,
                "posh derive macros do not support lifetimes",
            ));
        }
    }

    Ok(())
}

pub fn remove_domain_param(ident: &Ident, generics: &Generics) -> Result<Generics> {
    if generics.params.is_empty() {
        return Err(Error::new_spanned(
            ident,
            "posh expects type to be generic in domain",
        ));
    }

    let params = generics.params.iter().skip(1).cloned().collect();

    Ok(Generics {
        params,
        ..generics.clone()
    })
}

pub fn get_domain_param(ident: &Ident, generics: &Generics) -> Result<Ident> {
    let first_param = generics
        .params
        .first()
        .ok_or_else(|| Error::new_spanned(ident, "posh expects type to be generic in domain"))?;

    match first_param {
        GenericParam::Type(type_param) => Ok(type_param.ident.clone()),
        _ => Err(Error::new_spanned(
            first_param,
            "posh expects the first generic parameter to be the domain",
        )),
    }
}
