use proc_macro2::{TokenStream, Span};
use quote::quote;
use syn::{DeriveInput, Data, Fields, Type, Ident};

pub fn derive_unpack(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).expect("Could not parse derive input");
    let name: Ident = input.ident;

    let unpack_impl = match input.data {
        Data::Struct(s) => match s.fields {
            Fields::Named(fs) => {
                let (fields, tys): (Vec<Ident>, Vec<Type>) = fs.named.into_iter()
                    .map(|f| (f.ident.expect("#[derive(Unpack)] requires fields to be named"), f.ty))
                    .unzip();
                quote! {
                    #(let #fields = #tys::unpack::<B>(buffer)?; let buffer = &buffer[#tys::SIZE..];)*
                    Ok(#name { #(#fields),* })
                }
            },
            Fields::Unnamed(fs) => {
                let vars: Vec<Ident> = (0..fs.unnamed.len()).map(|i| Ident::new(&format!("x{}", i), Span::call_site())).collect();
                let tys: Vec<Type> = fs.unnamed.iter().map(|f| f.ty.clone()).collect();
                quote! {
                    #(let #vars = #tys::unpack::<B>(buffer)?; let buffer = &buffer[#tys::SIZE..];)*
                    Ok(#name(#(#vars),*))
                }
            },
            Fields::Unit => quote! { Ok(#name) },
        },
        Data::Enum(_) => unimplemented!("#[derive(Unpack)] is not supported for enums yet!"),
        Data::Union(_) => unimplemented!("#[derive(Unpack)] is not supported for unions yet!"),
    };

    // TODO: Handle generics
    quote! {
        impl ::lightpack::Unpack for #name {
            fn unpack<B>(buffer: &[u8]) -> ::lightpack::unpack::Result<Self> where B: ::lightpack::byteorder::ByteOrder {
                #unpack_impl
            }
        }
    }
}
