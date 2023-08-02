use proc_macro2::Ident;
use syn::{Type, DeriveInput, TypePath, Path};

/// Fetches the `#[repr(...)]` type.
pub fn repr_type(input: &DeriveInput) -> Option<Type> {
    let repr_attr = input.attrs.iter().find(|a| a.path().is_ident("repr"))?;

    let mut repr_type_opt: Option<Type> = None;
    repr_attr.parse_nested_meta(|meta| {
        repr_type_opt = Some(Type::Path(TypePath { qself: None, path: meta.path }));
        Ok(())
    }).ok()?;

    repr_type_opt
}

/// Converts a simple (non-parameterized, unqualified) type to the corresponding ident.
pub fn type_to_ident(t: &Type) -> Option<&Ident> {
    match t {
        Type::Path(TypePath { qself: None, path: Path { leading_colon: None, segments } })
            if segments.len() == 1 && segments[0].arguments.is_none() => {
            Some(&segments[0].ident)
        },
        _ => None,
    }
}
