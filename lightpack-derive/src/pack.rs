use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields, Index, Ident, Type};

use crate::util::{repr_type, type_to_turbofish};

pub fn derive_pack(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).expect("Could not parse derive input");
    let name: &Ident = &input.ident;

    let pack_impl = match &input.data {
        Data::Struct(s) => {
            let (fields, tys) = match &s.fields {
                Fields::Named(fs) => fs.named.iter()
                    .map(|f| {
                        let ident = f.ident.as_ref().expect("#[derive(Pack)] requires fields to be named");
                        (quote! { self.#ident }, &f.ty)
                    })
                    .unzip(),
                Fields::Unnamed(fs) => fs.unnamed.iter()
                    .enumerate()
                    .map(|(i, f)| {
                        // We need to do this, otherwise the quoter will append
                        // a number literal suffix, which isn't supported.
                        let index = Index::from(i);
                        (quote! { self.#index }, &f.ty)
                    })
                    .unzip(),
                Fields::Unit => (Vec::new(), Vec::new()),
            };

            let turbofish_tys = tys.into_iter()
                .cloned()
                .map(type_to_turbofish);

            quote! {
                #(#fields.pack::<B>(buffer); let buffer = &mut buffer[<#turbofish_tys>::SIZE..];)*
            }
        },
        Data::Enum(_) => {
            let repr_type: Type = repr_type(&input)
                .expect("#[derive(Pack)] currently only supports enums with a #[repr]");
            
            // TODO: Verify that enum also derives Copy?

            quote! {
                (*self as #repr_type).pack::<B>(buffer)
            }
        },
        Data::Union(_) => unimplemented!("#[derive(Pack)] is not supported for unions yet!"),
    };

    let impl_type_params = {
        let types = input.generics.type_params();
        quote! { <#(#types,)*> }
    };

    let where_clause = {
        let types = input.generics.type_params();
        quote! { #(#types: ::lightpack::Pack,)* }
    };

    quote! {
        impl #impl_type_params ::lightpack::Pack for #name #impl_type_params where #where_clause {
            fn pack<B>(&self, buffer: &mut [u8]) where B: ::lightpack::byteorder::ByteOrder {
                #pack_impl
            }
        }
    }
}
