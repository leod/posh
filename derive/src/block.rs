use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Ident, Result};

use crate::utils::{
    remove_view_param, specialize_field_types, SpecializedTypeGenerics, StructFields,
};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    let ident = &input.ident;
    let ident_str = ident.to_string();
    let visibility = input.vis;

    let as_std140_ident = Ident::new(&format!("PoshInternal{ident}BlockAsStd140"), ident.span());

    let generics_tail = remove_view_param(ident, &input.generics)?;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (impl_generics_init, ty_generics_init, where_clause_init) = generics_tail.split_for_impl();

    let ty_generics_logical =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Logical), ident, &input.generics)?;
    let ty_generics_physical =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Physical), ident, &input.generics)?;

    let fields = StructFields::new(&input.ident, &input.data)?;
    let field_idents = fields.idents();
    let field_strings = fields.strings();

    let field_types_logical = specialize_field_types(
        parse_quote!(::posh::Logical),
        ident,
        &input.generics,
        &fields,
    )?;
    let field_types_physical = specialize_field_types(
        parse_quote!(::posh::Physical),
        ident,
        &input.generics,
        &fields,
    )?;

    Ok(quote! {
        // Implement `Object` for the `Logical` view of the struct.
        impl #impl_generics_init ::posh::sl::Object for #ident #ty_generics_logical
        #where_clause_init
        {
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

        // Implement `Value` for the logical view of the struct.
        impl #impl_generics_init ::posh::sl::Value for #ident #ty_generics_logical
        #where_clause_init
        {
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

        // Implement `ValueNonArray` for the logical view of the struct.
        impl #impl_generics_init ::posh::sl::ValueNonArray for #ident #ty_generics_logical
        #where_clause
        {}

        // Implement `Struct` for the logical view of the struct.
        impl #impl_generics_init ::posh::sl::Struct for #ident #ty_generics_logical
        #where_clause
        {
            fn struct_type() -> ::std::rc::Rc<::posh::internal::StructType> {
                ::posh::internal::unique_struct_type::<Self>(
                    || ::posh::internal::StructType {
                        name: #ident_str.to_string(),
                        fields: vec![
                            #(
                                (
                                    #field_strings.to_string(),
                                    <#field_types_logical as ::posh::sl::Object>::ty(),
                                )
                            ),*
                        ],
                    }
                )

            }
        }

        // Implement `ToValue` for all views of the struct.
        impl #impl_generics ::posh::sl::ToValue for #ident #ty_generics
        #where_clause
        {
            type Output = #ident #ty_generics_logical;

            fn to_value(self) -> Self::Output {
                Self::Output {
                    #(
                        #field_idents: ::posh::sl::ToValue::to_value(self.#field_idents)
                    ),*
                }
            }
        }
        // Helper type for which we can derive `AsStd140`.
        // FIXME: AFAIK, crevice does not support generic types (yet?).
        #[doc(hidden)]
        #[derive(::posh::crevice::std140::AsStd140)]
        #visibility struct #as_std140_ident #impl_generics_init {
            #(
                #field_idents: #field_types_physical
            ),*
        }

        // Implement `AsStd140` for the physical view of the struct via the helper type above.
        impl #impl_generics_init ::posh::crevice::std140::AsStd140 for #ident #ty_generics_physical
        #where_clause
        {
            type Output = <
                #as_std140_ident #ty_generics_init as ::posh::crevice::std140::AsStd140
            >::Output;

            fn as_std140(&self) -> Self::Output {
                #as_std140_ident {
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

        // Implement `Block<Physical>` for the physical view of the struct.
        unsafe impl #impl_generics_init ::posh::Block<::posh::Physical>
        for #ident #ty_generics_physical
        #where_clause_init
        {
            type Logical = #ident #ty_generics_logical;
            type Physical = #ident #ty_generics_physical;

            fn uniform_input(path: &str) -> Self {
                unimplemented!()
            }

            fn vertex_input(path: &str) -> Self {
                unimplemented!()
            }
        }

        // Implement `Block<Logical>` for the logical view of the struct.
        unsafe impl #impl_generics_init ::posh::Block<::posh::Logical>
        for #ident #ty_generics_logical
        #where_clause_init
        {
            type Logical = #ident #ty_generics_logical;
            type Physical = #ident #ty_generics_physical;

            fn uniform_input(path: &str) -> Self {
                ::posh::internal::value_arg(path)
            }

            fn vertex_input(path: &str) -> Self {
                Self {
                    #(
                        #field_idents: <#field_types_logical as ::posh::Block<::posh::Logical>>::
                            vertex_input(
                                &::posh::internal::join_ident_path(path, #field_strings),
                            ),
                    )*
                }
            }

            fn vertex_attribute_defs(path: &str) -> Vec<::posh::util::VertexAttributeDef> {
                let mut result = Vec::new();

                // Passing this type to `offset_of` directly didn't work for me.
                type Pod = <
                    #as_std140_ident #ty_generics_init as ::posh::crevice::std140::AsStd140
                >::Output;

                #(
                    let offset = ::posh::bytemuck::offset_of!(
                        ::posh::bytemuck::Zeroable::zeroed(),
                        Pod,
                        #field_idents
                    );

                    let attrs = <
                        #field_types_logical as ::posh::Block<::posh::Logical>
                    >::vertex_attribute_defs(
                        &::posh::internal::join_ident_path(path, #field_strings),
                    );

                    for attr in attrs {
                        result.push(::posh::util::VertexAttributeDef {
                            offset: attr.offset + offset,
                            ..attr
                        });
                    }
                )*

                result
            }

        }

        // Check that all field types in `Logical` implement `Block<Logical>`.
        const _: fn() = || {
            fn check_field<T: ::posh::Block<::posh::Logical>>() {}

            fn check_struct #impl_generics(value: &#ident #ty_generics) #where_clause {
                #(
                    check_field::<#field_types_logical>();
                )*
            }
        };

        // Check that all field types in `Physical` implement `Block<Physical>`.
        const _: fn() = || {
            fn check_field<T: ::posh::Block<::posh::Physical>>() {}

            fn check_struct #impl_generics(value: &#ident #ty_generics) #where_clause {
                #(
                    check_field::<#field_types_physical>();
                )*
            }
        };
    })
}
