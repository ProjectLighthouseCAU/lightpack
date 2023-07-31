use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, Fields, Ident};

#[proc_macro_derive(Size)]
pub fn derive_size(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).expect("Could not parse derive input");
    let name = &input.ident;

    let fields: Vec<Ident> = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(fs) => fs.named.into_iter()
                .map(|f| f.ident.expect("#[derive(Size)] requires all fields to be named."))
                .collect(),
            Fields::Unnamed(_) => unimplemented!("#[derive(Size)] is not supported for tuple structs yet!"),
            Fields::Unit => Vec::new(),
        },
        Data::Enum(_) => unimplemented!("#[derive(Size)] is not supported for enums yet!"),
        Data::Union(_) => unimplemented!("#[derive(Size)] is not supported for unions yet!"),
    };

    let size_expr = fields.into_iter()
        .map(|f| quote! { #f::SIZE })
        .reduce(|e1, e2| quote! { #e1 + #e2 });

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
    todo!()
}

#[proc_macro_derive(Unpack)]
pub fn derive_unpack(input: TokenStream) -> TokenStream {
    todo!()
}
