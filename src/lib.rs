extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_roids::{DeriveInputStructExt, FieldExt, IdentExt};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

/// Derives a `Super` enum with a variant for each struct field:
///
/// ```rust,edition2018
/// use std::marker::PhantomData;
/// use super_derive::Super;
///
/// #[derive(Super)]
/// pub struct Man<T> {
///     #[super_derive(skip)]
///     name: String,
///     power_level: u64,
///     marker: PhantomData<T>,
/// }
/// ```
///
/// Generates:
///
/// ```rust,ignore
/// pub enum SuperMan {
///     U64(u64),
/// }
/// ```
#[proc_macro_derive(Super, attributes(super_derive))]
pub fn system_desc_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let enum_name = ast.ident.prepend("Super");
    let fields = ast.fields();
    let relevant_fields = fields
        .iter()
        .filter(|field| !field.is_phantom_data())
        .filter(|field| !field.contains_tag("super_derive", "skip"));

    let variants = relevant_fields
        .map(|field| {
            let type_name = field.type_name();
            let variant_name = type_name.to_string().to_uppercase();
            let variant_name = Ident::new(&variant_name, Span::call_site());
            quote! {
                #variant_name(#type_name)
            }
        })
        .collect::<Vec<_>>();

    let token_stream2 = quote! {
        pub enum #enum_name {
            #(#variants,)*
        }
    };

    token_stream2.into()
}
