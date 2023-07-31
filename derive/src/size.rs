use proc_macro2::TokenStream;
use syn::{DeriveInput, Ident, Type, Data, Fields};
use quote::quote;

use crate::util::repr_type;

pub fn derive_size(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).expect("Could not parse derive input");
    let name: &Ident = &input.ident;

    let size_expr = match input.data {
        Data::Struct(s) => {
            let field_types: Vec<Type> = match s.fields {
                Fields::Named(fs) => fs.named.into_iter()
                    .map(|f| f.ty)
                    .collect(),
                Fields::Unnamed(fs) => fs.unnamed.into_iter()
                    .map(|f| f.ty)
                    .collect(),
                Fields::Unit => Vec::new(),
            };

            field_types.into_iter()
                .map(|t| quote! { #t::SIZE })
                .reduce(|e1, e2| quote! { #e1 + #e2 })
                .unwrap_or_else(|| quote! { 0 })
        },
        Data::Enum(_) => {
            let repr_type: Type = repr_type(&input)
                .expect("#[derive(Size)] currently only supports enums with a #[repr]");

            quote! {
                #repr_type::SIZE
            }
        },
        Data::Union(_) => unimplemented!("#[derive(Size)] is not supported for unions yet!"),
    };

    // TODO: Handle generics
    quote! {
        impl ::lightpack::Size for #name {
            const SIZE: usize = #size_expr;
        }
    }
}