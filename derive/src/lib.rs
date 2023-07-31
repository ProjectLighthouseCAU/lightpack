use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Data, Fields, Type, Index, Ident};

#[proc_macro_derive(Size)]
pub fn derive_size(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).expect("Could not parse derive input");
    let name: Ident = input.ident;

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

#[proc_macro_derive(Unpack)]
pub fn derive_unpack(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).expect("Could not parse derive input");
    let name: Ident = input.ident;

    let unpack_impl = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(fs) => {
                let fields: Vec<Ident> = fs.named.iter().map(|f| f.ident.clone().expect("#[derive(Unpack)] requires fields to be named")).collect();
                let tys: Vec<Type> = fs.named.iter().map(|f| f.ty.clone()).collect();
                quote! {
                    #(let #fields = #tys::unpack::<B>(buffer); let buffer = &buffer[#tys::SIZE..];)*
                    #name { #(#fields),* }
                }
            },
            Fields::Unnamed(fs) => {
                let vars: Vec<Ident> = (0..fs.unnamed.len()).map(|i| Ident::new(&format!("x{}", i), Span::call_site())).collect();
                let tys: Vec<Type> = fs.unnamed.iter().map(|f| f.ty.clone()).collect();
                quote! {
                    #(let #vars = #tys::unpack::<B>(buffer); let buffer = &buffer[#tys::SIZE..];)*
                    #name(#(#vars),*)
                }
            },
            Fields::Unit => quote! { #name },
        },
        Data::Enum(_) => unimplemented!("#[derive(Unpack)] is not supported for enums yet!"),
        Data::Union(_) => unimplemented!("#[derive(Unpack)] is not supported for unions yet!"),
    };

    // TODO: Handle generics
    let impl_block = quote! {
        impl ::lightpack::Unpack for #name {
            fn unpack<B>(buffer: &[u8]) -> Self where B: ::lightpack::byteorder::ByteOrder {
                #unpack_impl
            }
        }
    };

    impl_block.into()
}
