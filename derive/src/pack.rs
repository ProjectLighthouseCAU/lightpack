use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields, Index, Ident};

pub fn derive_pack(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).expect("Could not parse derive input");
    let name: Ident = input.ident;

    let field_exprs_and_tys: Vec<(_, _)> = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(fs) => fs.named.into_iter()
                .map(|f| {
                    let ident = f.ident.expect("#[derive(Pack)] requires fields to be named");
                    (quote! { self.#ident }, f.ty)
                })
                .collect(),
            Fields::Unnamed(fs) => fs.unnamed.into_iter()
                .enumerate()
                .map(|(i, f)| {
                    // We need to do this, otherwise the quoter will append
                    // a number literal suffix, which isn't supported.
                    let index = Index::from(i);
                    (quote! { self.#index }, f.ty)
                })
                .collect(),
            Fields::Unit => Vec::new(),
        },
        Data::Enum(_) => unimplemented!("#[derive(Pack)] is not supported for enums yet!"),
        Data::Union(_) => unimplemented!("#[derive(Pack)] is not supported for unions yet!"),
    };

    let field_pack_impls: Vec<_> = field_exprs_and_tys.into_iter()
        .map(|(e, t)| quote! { #e.pack::<B>(buffer); let buffer = &mut buffer[#t::SIZE..]; })
        .collect();

    // TODO: Handle generics
    let impl_block = quote! {
        impl ::lightpack::Pack for #name {
            fn pack<B>(&self, buffer: &mut [u8]) where B: ::lightpack::byteorder::ByteOrder {
                #(#field_pack_impls)*
            }
        }
    };

    impl_block.into()
}
