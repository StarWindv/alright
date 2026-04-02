use proc_macro::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, Field, Fields};
use syn::punctuated::Punctuated;
use syn::token::Comma;


pub(crate) fn implement(struct_name: &syn::Ident, property_name: &syn::Ident) -> TokenStream {
    TokenStream::from(quote! {
        impl Transform<#struct_name> for #struct_name {}

        impl ExceptionUtils<#struct_name> for #struct_name {
            fn get_property(&self) -> Box<Property<#struct_name>> {
                self.#property_name.clone()
            }
            fn set_property(&mut self, property: Box<Property<#struct_name>>) {
                self.#property_name = property;
            }
            fn get_ptr(&self) -> #struct_name {
                self.clone()
            }
            fn set_ptr(&mut self, ptr: #struct_name) {
                *self = ptr;
            }
        }

        impl std::convert::From<BaseException<#struct_name>> for #struct_name {
            fn from(value: BaseException<#struct_name>) -> Self {
                #struct_name::up(value)
            }
        }

        impl std::error::Error for #struct_name {}
        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.#property_name.name)
            }
        }
    })
}

pub(crate) fn is_struct(input: &DeriveInput) -> Result<&DataStruct, TokenStream> {
    match &input.data {
        syn::Data::Struct(s) => Ok(s),
        _ => {
            Err(
                syn::Error::new_spanned(
                    &input,
                    "[Exception] Only for `struct`"
                ).to_compile_error()
                    .into()
            )
        }
    }
}

pub(crate) fn is_named_struct(data_struct: &DataStruct) -> Result<&Punctuated<Field, Comma>, TokenStream> {
    match &data_struct.fields {
        Fields::Named(fields) => Ok(&fields.named),
        _ => {
            Err(
                syn::Error::new_spanned(
                    &data_struct.fields,
                    "[Exception] Only for Named Struct",
                )
                    .to_compile_error()
                    .into()
            )
        }
    }
}

pub(crate) fn get_property_field_name(
    matching_fields: Vec<&Field>,
    struct_name: &syn::Ident
) -> Result<proc_macro2::Ident, TokenStream> {
    match matching_fields.len() {
        1 => Ok(matching_fields[0].ident.clone().unwrap()),
        0 => {
            Err(
                syn::Error::new_spanned(
                    &struct_name,
                    format!("[Exception] Required One Field Has this Type: \n - Box<Property<{struct_name}>>"),
                )
                    .to_compile_error()
                    .into()
            )
        }
        _ => {
            Err(
                syn::Error::new_spanned(
                    &matching_fields[1],
                    format!("[Exception] Only Support One Field Has Type: \n - Box<Property<{struct_name}>>"),
                )
                    .to_compile_error()
                    .into()
            )
        }
    }
}
