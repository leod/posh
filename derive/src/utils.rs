use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_quote, punctuated::Punctuated, Data, Error, Field, Fields, GenericParam, Generics, Ident,
    Result, Token, Type, TypeParamBound,
};

#[derive(Clone)]
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

pub struct SpecializedTypeGenerics {
    domain: Type,
    params: Vec<GenericParam>,
}

impl SpecializedTypeGenerics {
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

impl ToTokens for SpecializedTypeGenerics {
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

pub struct SpecializeFieldTypesConfig {
    pub context: &'static str,
    pub domain: Type,
    pub bounds: Punctuated<TypeParamBound, Token![+]>,
    pub map_trait: Type,
    pub map_type: Type,
}

pub fn specialize_field_types(
    config: SpecializeFieldTypesConfig,
    ident: &Ident,
    generics: &Generics,
    fields: &StructFields,
) -> Result<(TokenStream, Vec<Type>)> {
    let context = config.context;
    let helper_trait = Ident::new(
        &format!("PoshInternal{ident}{context}SpecializeFields"),
        ident.span(),
    );

    let bounds = &config.bounds;
    let map_trait = &config.map_trait;
    let map_type = &config.map_type;

    let ident = ident;
    let helper_trait_ident = &helper_trait;

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let specialized_ty_generics = SpecializedTypeGenerics::new(config.domain, ident, generics)?;

    let field_idents = fields.idents();
    let field_types = fields.types();

    let setup = quote! {
        // Helper trait for specializing struct field types for the given domain.
        #[doc(hidden)]
        trait #helper_trait_ident {
            #(
                #[allow(non_camel_case_types)]
                type #field_idents: #bounds;
            )*
        }

        // Implement the helper trait.
        impl #impl_generics #helper_trait_ident for #ident #ty_generics
        #where_clause
        {
            #(
                type #field_idents = <#field_types as #map_trait>::#map_type;
            )*
        }
    };

    let specialized_types = field_idents
        .iter()
        .map(|field_ident| {
            parse_quote! {
                <#ident #specialized_ty_generics as #helper_trait_ident>::#field_ident
            }
        })
        .collect();

    Ok((setup, specialized_types))
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
