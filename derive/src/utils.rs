use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote,
    spanned::Spanned,
    visit_mut::{visit_type_mut, VisitMut},
    Attribute, Data, Error, Field, Fields, GenericParam, Generics, Ident, Path, QSelf, Result,
    Token, Type, TypePath,
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

    pub fn attrs(&self) -> Vec<&[Attribute]> {
        self.fields
            .iter()
            .map(|field| field.attrs.as_slice())
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
                    panic!("internal error: posh does not support lifetimes");
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

pub fn associated_type_to_trait(ty: &str) -> Option<Path> {
    let block_tys = vec![
        "F32", "I32", "U32", "Vec2", "Vec3", "Vec4", "IVec2", "IVec3", "IVec4", "UVec2", "UVec3",
        "UVec4", "Mat2", "Mat3", "Mat4",
    ];

    let vertex_tys = vec!["Block"];

    let fragment_tys = vec!["ColorAttachment"];

    // TODO: Update associated type list, or see if we can do without it by now.

    if block_tys.contains(&ty) {
        Some(parse_quote!(::posh::BlockDom))
    } else if vertex_tys.contains(&ty) {
        Some(parse_quote!(::posh::VsInterfaceDom))
    } else if fragment_tys.contains(&ty) {
        Some(parse_quote!(::posh::FsInterfaceDom))
    } else {
        None
    }
}

pub fn specialize_field_types(
    domain: Path,
    ident: &Ident,
    generics: &Generics,
    fields: &StructFields,
) -> Result<Vec<Type>> {
    struct Visitor {
        domain: Path,
        generics_d_ident: Ident,
    }

    impl VisitMut for Visitor {
        fn visit_type_path_mut(&mut self, i: &mut TypePath) {
            self.visit_path_mut(&mut i.path);

            if let Some(qself) = i.qself.as_mut() {
                self.visit_qself_mut(qself);
                return;
            }

            if i.path.segments.is_empty() {
                return;
            }

            let first_segment = &i.path.segments[0];

            if first_segment.ident != self.generics_d_ident {
                return;
            }

            if i.path.segments.len() == 1 {
                i.path = self.domain.clone();
                return;
            }

            let Some(trait_path) = associated_type_to_trait(&i.path.segments[1].ident.to_string())
            else { return; };

            i.qself = Some(QSelf {
                lt_token: Token![<](first_segment.span()),
                ty: Box::new(Type::Path(TypePath {
                    qself: None,
                    path: self.domain.clone(),
                })),
                position: trait_path.segments.len(),
                as_token: Some(Token![as](first_segment.span())),
                gt_token: Token![>](first_segment.span()),
            });

            i.path.segments = trait_path
                .segments
                .into_iter()
                .chain(i.path.segments.clone().into_iter().skip(1))
                .collect();
        }
    }

    let mut visitor = Visitor {
        domain,
        generics_d_ident: get_domain_param(ident, generics)?,
    };

    let mut types = fields.types().into_iter().cloned().collect();

    for ty in &mut types {
        visit_type_mut(&mut visitor, ty);
    }

    Ok(types)
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
            "posh derive macro expects the struct to be generic in its domain",
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
            "posh derive macro expects the struct to be generic in its domain",
        )
    })?;

    match last_param {
        GenericParam::Type(type_param) => Ok(type_param.ident.clone()),
        _ => Err(Error::new_spanned(
            last_param,
            "posh derive macro expects the last generic parameter to be generic in the domain",
        )),
    }
}
