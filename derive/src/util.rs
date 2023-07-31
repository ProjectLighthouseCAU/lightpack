use syn::{Type, DeriveInput, TypePath};

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
