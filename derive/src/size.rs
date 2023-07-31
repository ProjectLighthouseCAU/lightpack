use proc_macro2::TokenStream;
use syn::{DeriveInput, Ident, Type, Data, Fields};
use quote::quote;

pub fn derive_size(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).expect("Could not parse derive input");
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
