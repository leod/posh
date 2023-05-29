use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Error, Ident, Result};

use crate::{
    utils::{
        remove_domain_param, specialize_field_types, validate_generics, SpecializedTypeGenerics,
        StructFields,
    },
    value,
};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    validate_generics(&input.generics)?;

    let attrs = &input.attrs;
    let ident = &input.ident;
    let visibility = input.vis;

    let helper_ident = Ident::new(&format!("PoshInternal{ident}BlockHelper"), ident.span());

    let generics_init = remove_domain_param(ident, &input.generics)?;

    if !generics_init.params.is_empty() {
        return Err(Error::new_spanned(
            &generics_init.params[0],
            "posh derive(Block) macro expects the struct to be generic only in its domain",
        ));
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (impl_generics_init, _, where_clause_init) = generics_init.split_for_impl();

    let ty_generics_sl =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Sl), ident, &input.generics)?;
    let ty_generics_gl =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Gl), ident, &input.generics)?;

    let fields = StructFields::new(&input.ident, &input.data)?;
    let field_idents = fields.idents();
    let field_strings = fields.strings();

    let field_types_sl =
        specialize_field_types(parse_quote!(::posh::Sl), ident, &input.generics, &fields)?;
    let field_types_gl =
        specialize_field_types(parse_quote!(::posh::Gl), ident, &input.generics, &fields)?;

    let value_impl = value::derive_impl(
        &ident.to_string(),
        &parse_quote!(#ident #ty_generics_sl),
        field_idents.as_slice(),
        &field_types_sl.iter().collect::<Vec<_>>(),
        (&impl_generics_init, where_clause_init),
    )?;

    Ok(quote! {
        // Implement `Value` and co. for the `Sl` view of the struct.
        #value_impl

        // Implement `ToSl` for all views of the struct.
        impl #impl_generics ::posh::sl::ToSl for #ident #ty_generics
        #where_clause
        {
            type Output = #ident #ty_generics_sl;

            fn to_sl(self) -> Self::Output {
                Self::Output {
                    #(
                        #field_idents: ::posh::sl::ToSl::to_sl(self.#field_idents)
                    ),*
                }
            }
        }

        // Helper type that is specialized for the `Gl` view of the struct. We
        // use this for two things:
        // 1. Derive `AsStd140` for the `Gl` view of the struct by utilizing the
        //    derive macro of `crevice`.
        // 2. Safety: check that the `Gl` view of the struct satisfies the
        //    requirements for `Zeroable` and `Pod` by utilizing the derive
        //    macros of `bytemuck`.
        #[doc(hidden)]
        #[derive(
            Clone,
            Copy,
            ::posh::bytemuck::Zeroable,
            ::posh::bytemuck::Pod,
            ::posh::crevice::std140::AsStd140,
        )]
        #[bytemuck_crate(::posh::bytemuck)]
        #[crevice_crate(::posh::crevice)]
        #(#attrs)*
        #visibility struct #helper_ident {
            #(
                #field_idents: #field_types_gl
            ),*
        }

        // Implement `Zeroable` for the `Gl` view of the struct.
        unsafe impl ::posh::bytemuck::Zeroable for #ident #ty_generics_gl {}

        // Implement `Pod` for the `Gl` view of the struct.
        unsafe impl ::posh::bytemuck::Pod for #ident #ty_generics_gl {}

        // Implement `AsStd140` for the `Gl` view of the struct via the helper
        // type above.
        impl ::posh::crevice::std140::AsStd140 for #ident #ty_generics_gl {
            type Output = <#helper_ident as ::posh::crevice::std140::AsStd140>::Output;

            fn as_std140(&self) -> Self::Output {
                #helper_ident {
                    #(
                        #field_idents: self.#field_idents.clone()
                    ),*
                }
                .as_std140()
            }

            fn from_std140(val: Self::Output) -> Self {
                Self {
                    #(
                        #field_idents: ::posh::crevice::std140::AsStd140::from_std140(
                            val.#field_idents,
                        )
                    ),*
                }
            }
        }

        // Implement `Block<Gl>` for the `Gl` view of the struct.
        unsafe impl ::posh::Block<::posh::Gl> for #ident #ty_generics_gl {
            type Sl = #ident #ty_generics_sl;
            type Gl = #ident #ty_generics_gl;

            fn uniform_input(path: &str) -> Self {
                unimplemented!()
            }

            fn vertex_input(path: &str) -> Self {
                unimplemented!()
            }
        }

        // Implement `Block<Sl>` for the `Sl` view of the struct.
        unsafe impl ::posh::Block<::posh::Sl> for #ident #ty_generics_sl
        {
            type Sl = #ident #ty_generics_sl;
            type Gl = #ident #ty_generics_gl;

            fn uniform_input(path: &str) -> Self {
                ::posh::internal::value_arg(path)
            }

            fn vertex_input(path: &str) -> Self {
                Self {
                    #(
                        #field_idents: <#field_types_sl as ::posh::Block<::posh::Sl>>::
                            vertex_input(
                                &::posh::internal::join_ident_path(path, #field_strings),
                            ),
                    )*
                }
            }

            fn vertex_attribute_defs(path: &str) -> Vec<::posh::sl::program_def::VertexAttributeDef>
            {
                let mut result = Vec::new();

                #(
                    let offset = ::posh::bytemuck::offset_of!(
                        ::posh::bytemuck::Zeroable::zeroed(),
                        Self::Gl,
                        #field_idents
                    );

                    let attrs = <
                        #field_types_sl as ::posh::Block<::posh::Sl>
                    >::vertex_attribute_defs(
                        &::posh::internal::join_ident_path(path, #field_strings),
                    );

                    for attr in attrs {
                        result.push(::posh::sl::program_def::VertexAttributeDef {
                            offset: attr.offset + offset,
                            ..attr
                        });
                    }
                )*

                result
            }
        }

        // Implement `Interpolant` for the `Sl` view of the struct.
        // TODO: This can go away once we unify `Value` and `Interpolant`.
        unsafe impl ::posh::sl::Interpolant for #ident #ty_generics_sl {
            fn shader_outputs(&self, path: &str) -> Vec<(
                ::std::string::String,
                ::posh::sl::program_def::InterpolationQualifier,
                ::std::rc::Rc<::posh::internal::Expr>,
            )> {
                let mut result = Vec::new();

                #(
                    result.extend(
                        <#field_types_sl as ::posh::sl::Interpolant>::shader_outputs(
                            &self.#field_idents,
                            &::posh::internal::join_ident_path(path, #field_strings)
                        )
                    );
                )*

                result
            }

            fn shader_input(path: &str) -> Self {
                Self {
                    #(
                        #field_idents: <#field_types_sl as ::posh::sl::Interpolant>::
                            shader_input(&::posh::internal::join_ident_path(path, #field_strings)),
                    )*
                }
            }
        }

        // Check that all field types in `Sl` implement `Block<Sl>`.
        const _: fn() = || {
            fn check_field<T: ::posh::Block<::posh::Sl>>() {}

            fn check_struct #impl_generics() #where_clause {
                #(
                    check_field::<#field_types_sl>();
                )*
            }
        };

        // Check that all field types in `Gl` implement `Block<Gl>`.
        const _: fn() = || {
            fn check_field<T: ::posh::Block<::posh::Gl>>() {}

            fn check_struct #impl_generics() #where_clause {
                #(
                    check_field::<#field_types_gl>();
                )*
            }
        };

        // Check that all field types in `Sl` implement `Interpolant`.
        // TODO: This can go away once we unify `Value` and `Interpolant`.
        const _: fn() = || {
            fn check_field<V: ::posh::sl::Interpolant>() {}

            fn check_struct #impl_generics() #where_clause {
                #(
                    check_field::<#field_types_sl>();
                )*
            }
        };
    })
}
