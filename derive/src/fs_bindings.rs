use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput, Result};

use crate::{
    utils::{
        get_domain_param, remove_domain_param, specialize_field_types, validate_generics,
        SpecializedTypeGenerics, StructFields,
    },
    value,
};

pub fn derive(input: DeriveInput) -> Result<TokenStream> {
    validate_generics(&input.generics)?;

    let ident = &input.ident;

    let generics_view_type = get_domain_param(ident, &input.generics)?;
    let generics_init = remove_domain_param(ident, &input.generics)?;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let (impl_generics_init, _, where_clause_init) = generics_init.split_for_impl();

    let ty_generics_sl =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Sl), ident, &input.generics)?;
    let ty_generics_gl =
        SpecializedTypeGenerics::new(parse_quote!(::posh::Gl), ident, &input.generics)?;

    let fields = StructFields::new(&input.ident, &input.data)?;
    let field_idents = fields.idents();
    let field_types = fields.types();
    let field_strings = fields.strings();

    let field_types_sl =
        specialize_field_types(parse_quote!(::posh::Sl), ident, &input.generics, &fields)?;

    let value_impl = value::derive_impl(
        &ident.to_string(),
        &parse_quote!(#ident #ty_generics_sl),
        field_idents.as_slice(),
        &field_types_sl.iter().collect::<Vec<_>>(),
        (&impl_generics_init, where_clause_init),
    )?;

    Ok(quote! {
        // Implement `FsBindings<D>` for the struct.
        unsafe impl #impl_generics ::posh::FsBindings<#generics_view_type>
        for #ident #ty_generics
        #where_clause
        {
            type Gl = #ident #ty_generics_gl;
            type Sl = #ident #ty_generics_sl;

            fn visit<'a>(
                &'a self,
                path: &str,
                visitor: &mut impl ::posh::internal::FragmentVisitor<'a, #generics_view_type>,
            ) {
                #(
                    visitor.accept(
                        &::posh::internal::join_ident_path(path, #field_strings),
                        &self.#field_idents,
                    );
                )*
            }
        }

        // Implement `Value` and co. for the `Sl` view of the struct.
        #value_impl

        // Implement `ToSl` for the struct.
        impl #impl_generics_init ::posh::sl::ToSl for #ident #ty_generics_sl #where_clause_init {
            type Output = Self;

            fn to_sl(self) -> Self {
                self
            }
        }

        // Implement `Varying` for the `Sl` view of the struct.
        // TODO: This can go away once we unify `Value` and `Varying`.
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

        // Check that all field types implement `FsBindings<D>`.
        const _: fn() = || {
            fn check_field<D, T>()
            where
                D: ::posh::FsBindingsDom,
                T: ::posh::FsBindings<D>,
            {}

            fn check_struct #impl_generics(value: &#ident #ty_generics) #where_clause {
                #(
                    check_field::<#generics_view_type, #field_types>();
                )*
            }
        };

        // Check that all field types in `Sl` implement `Varying`.
        // TODO: This can go away once we unify `Value` and `Varying`.
        const _: fn() = || {
            fn check_field<V: ::posh::sl::Varying>() {}

            fn check_struct #impl_generics() #where_clause {
                #(
                    check_field::<#field_types_sl>();
                )*
            }
        };
    })
}
