use proc_macro2::{TokenStream, Span};
use quote::quote;
use syn::{DeriveInput, Data, Fields, Type, Ident, Expr};

use crate::util::{repr_type, type_to_ident, type_to_turbofish};

pub fn derive_unpack(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse2(input).expect("Could not parse derive input");
    let name: &Ident = &input.ident;

    let unpack_impl = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(fs) => {
                let (fields, turbofish_tys): (Vec<&Ident>, Vec<Type>) = fs.named.iter()
                    .map(|f| (f.ident.as_ref().expect("#[derive(Unpack)] requires fields to be named"), type_to_turbofish(f.ty.clone())))
                    .unzip();
                quote! {
                    #(let #fields = #turbofish_tys::unpack::<B>(buffer)?; let buffer = &buffer[#turbofish_tys::SIZE..];)*
                    Ok(#name { #(#fields),* })
                }
            },
            Fields::Unnamed(fs) => {
                let vars: Vec<Ident> = (0..fs.unnamed.len()).map(|i| Ident::new(&format!("x{}", i), Span::call_site())).collect();
                let turbofish_tys: Vec<Type> = fs.unnamed.iter().map(|f| type_to_turbofish(f.ty.clone())).collect();
                quote! {
                    #(let #vars = #turbofish_tys::unpack::<B>(buffer)?; let buffer = &buffer[#turbofish_tys::SIZE..];)*
                    Ok(#name(#(#vars),*))
                }
            },
            Fields::Unit => quote! { Ok(#name) },
        },
        Data::Enum(ref e) => {
            let repr_type: Type = repr_type(&input)
                .expect("#[derive(Unpack)] currently only supports enums with a #[repr]");
            let repr_ident: &Ident = type_to_ident(&repr_type)
                .expect("#[derive(Unpack)] currently only supports enums with a primitive #[repr]");
            
            // TODO: Verify that enum also derives Copy?
            
            let (variants, discriminants): (Vec<&Ident>, Vec<&Expr>) = e.variants.iter()
                .map(|v| (&v.ident, &v.discriminant.as_ref().expect("#[derive(Unpack)] requires explicit enum discriminants (for now)").1))
                .unzip();

            let error_variant = Ident::new(
                &format!("InvalidEnumValue{}", repr_ident.to_string().to_uppercase()),
                Span::call_site()
            );

            quote! {
                match #repr_type::unpack::<B>(buffer)? {
                    #(#discriminants => Ok(Self::#variants),)*
                    value => Err(::lightpack::unpack::Error::#error_variant(value)),
                }
            }
        },
        Data::Union(_) => unimplemented!("#[derive(Unpack)] is not supported for unions yet!"),
    };

    let impl_type_params = {
        let types = input.generics.type_params();
        quote! { <#(#types,)*> }
    };

    let where_clause = {
        let types = input.generics.type_params();
        quote! { #(#types: ::lightpack::Unpack,)* }
    };

    quote! {
        impl #impl_type_params ::lightpack::Unpack for #name #impl_type_params where #where_clause {
            fn unpack<B>(buffer: &[u8]) -> ::lightpack::unpack::Result<Self> where B: ::lightpack::byteorder::ByteOrder {
                #unpack_impl
            }
        }
    }
}
