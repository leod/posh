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
    domain: Type,
    params: Vec<GenericParam>,
}

impl SpecializeDomain {
    pub fn new(domain: Type, ident: &Ident, generics: &Generics) -> Result<Self> {
        Ok(Self {
            domain,
            params: remove_domain_param(ident, generics)?
                .params
                .into_iter()
                .collect(),
        })
    }
}

impl ToTokens for SpecializeDomain {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        <Token![<]>::default().to_tokens(tokens);

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

        self.domain.to_tokens(tokens);

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
            "posh derive macro expects type to be generic in `Domain`",
        ));
    }

    let mut params = generics.params.clone();
    params.pop();

    Ok(Generics {
        params,
        ..generics.clone()
    })
}

pub fn get_domain_param(ident: &Ident, generics: &Generics) -> Result<Ident> {
    let last_param = generics.params.last().ok_or_else(|| {
        Error::new_spanned(
            ident,
            "posh derive macro expects type to be generic in `Domain`",
        )
    })?;

    match last_param {
        GenericParam::Type(type_param) => Ok(type_param.ident.clone()),
        _ => Err(Error::new_spanned(
            last_param,
            "posh derive macro expects the last generic parameter to be generic in `Domain`",
        )),
    }
}
