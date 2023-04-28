use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Error, Ident, Result};

use crate::utils::{
    remove_domain_param, specialize_field_types, SpecializedTypeGenerics, StructFields,
};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let attrs = &input.attrs;
    let ident = &input.ident;
    let ident_str = ident.to_string();
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

    Ok(quote! {
        // Implement `Object` for the `Sl` view of the struct.
        impl ::posh::sl::Object for #ident #ty_generics_sl {
            fn ty() -> ::posh::internal::Type {
                ::posh::internal::Type::Struct(<Self as ::posh::sl::Struct>::struct_type())
            }

            fn expr(&self) -> ::std::rc::Rc<::posh::internal::Expr> {
                ::posh::internal::simplify_struct_literal(
                    <Self as ::posh::sl::Struct>::struct_type(),
                    &[
                        #(
                            ::posh::sl::Object::expr(&self.#field_idents)
                        ),*
                    ]
                )
            }

            fn from_arg(name: &str) -> Self {
                ::posh::internal::value_arg(name)
            }
        }

        // Implement `Value` for the `Sl` view of the struct.
        impl ::posh::sl::Value for #ident #ty_generics_sl {
            fn from_expr(expr: ::posh::internal::Expr) -> Self {
                let base = ::std::rc::Rc::new(expr);

                Self {
                    #(
                        #field_idents: ::posh::internal::field(
                            base.clone(),
                            #field_strings,
                        )
                    ),*
                }
            }
        }

        // Implement `ValueNonArray` for the `Sl` view of the struct.
        impl ::posh::sl::ValueNonArray for #ident #ty_generics_sl {}

        // Implement `Struct` for the `Sl` view of the struct.
        impl ::posh::sl::Struct for #ident #ty_generics_sl {
            fn struct_type() -> ::std::rc::Rc<::posh::internal::StructType> {
                ::posh::internal::unique_struct_type::<Self>(
                    || ::posh::internal::StructType {
                        name: #ident_str.to_string(),
                        fields: vec![
                            #(
                                (
                                    #field_strings.to_string(),
                                    <#field_types_sl as ::posh::sl::Object>::ty(),
                                )
                            ),*
                        ],
                    }
                )

            }
        }

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

        // Implement `Varying` for the `Sl` view of the struct.
        unsafe impl ::posh::sl::Varying for #ident #ty_generics_sl {
            fn shader_outputs(&self, path: &str) -> Vec<(
                ::std::string::String,
                ::posh::sl::program_def::InterpolationQualifier,
                ::std::rc::Rc<::posh::internal::Expr>,
            )> {
                let mut result = Vec::new();

                #(
                    result.extend(
                        <#field_types_sl as ::posh::sl::Varying>::shader_outputs(
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
                        #field_idents: <#field_types_sl as ::posh::sl::Varying>::
                            shader_input(&::posh::internal::join_ident_path(path, #field_strings)),
                    )*
                }
            }
        }

        // Check that all field types in `Sl` implement `Block<Sl>`.
        const _: fn() = || {
            fn check_field<T: ::posh::Block<::posh::Sl>>() {}

            fn check_struct #impl_generics(value: &#ident #ty_generics) #where_clause {
                #(
                    check_field::<#field_types_sl>();
                )*
            }
        };

        // Check that all field types in `Gl` implement `Block<Gl>`.
        const _: fn() = || {
            fn check_field<T: ::posh::Block<::posh::Gl>>() {}

            fn check_struct #impl_generics(value: &#ident #ty_generics) #where_clause {
                #(
                    check_field::<#field_types_gl>();
                )*
            }
        };
    })
}
