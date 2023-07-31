mod pack;
mod size;
mod unpack;

use proc_macro::TokenStream;

#[proc_macro_derive(Size)]
pub fn derive_size(input: TokenStream) -> TokenStream {
    size::derive_size(input.into()).into()
}

#[proc_macro_derive(Pack)]
pub fn derive_pack(input: TokenStream) -> TokenStream {
    pack::derive_pack(input.into()).into()
}

#[proc_macro_derive(Unpack)]
pub fn derive_unpack(input: TokenStream) -> TokenStream {
    unpack::derive_unpack(input.into()).into()
}
