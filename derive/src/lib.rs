use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields, Type, Ident, Index};

#[proc_macro_derive(Size)]
pub fn derive_size(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).expect("Could not parse derive input");
    let name = &input.ident;

    let field_types: Vec<Type> = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(fs) => fs.named.into_iter()
                .map(|f| f.ty)
                .collect(),
            Fields::Unnamed(fs) => fs.unnamed.into_iter()
                .map(|f| f.ty)
                .collect(),
            Fields::Unit => Vec::new(),
        },
        Data::Enum(_) => unimplemented!("#[derive(Size)] is not supported for enums yet!"),
        Data::Union(_) => unimplemented!("#[derive(Size)] is not supported for unions yet!"),
    };

    let size_expr = field_types.into_iter()
        .map(|t| quote! { #t::SIZE })
        .reduce(|e1, e2| quote! { #e1 + #e2 })
        .unwrap_or_else(|| quote! { 0 });

    // TODO: Handle generics
    let impl_block = quote! {
        impl ::lightpack::Size for #name {
            const SIZE: usize = #size_expr;
        }
    };

    impl_block.into()
}

#[proc_macro_derive(Pack)]
pub fn derive_pack(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).expect("Could not parse derive input");
    let name = &input.ident;

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

#[proc_macro_derive(Unpack)]
pub fn derive_unpack(input: TokenStream) -> TokenStream {
    todo!()
}
