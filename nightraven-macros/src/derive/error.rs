use syn::{spanned::Spanned, DeriveInput, Expr, MetaNameValue};

use super::{Attributes, Identifier};

/// `Asset` errors enumeration.
pub enum Error<'a> {
    Attributes(&'a Attributes),
    Attribute(&'a MetaNameValue),
    Expr(&'a Expr),
    Identifier(Option<String>),
    Data(&'a DeriveInput),
}

impl<'a> From<Error<'a>> for syn::Error {
    /// Converts custom `Asset` error into `syn::Error`.
    fn from(value: Error) -> Self {
        match value {
            Error::Attributes(attr) => syn::Error::new(
                quote::__private::Span::call_site(),
                format!(
                    "expected `{}(..., ...)`, got {:#?}",
                    Attributes::base(),
                    attr
                ),
            ),
            Error::Expr(expr) => syn::Error::new(
                expr.span(),
                format!(
                    "expected valid expression `{}(path = \"expression\")`, got {:#?}",
                    Attributes::base(),
                    expr
                ),
            ),
            Error::Attribute(meta) => syn::Error::new(
                meta.span(),
                format!("expected `sub_attribute = \" ... \"`, got {meta:#?}"),
            ),
            // This one is quite a struggle... definitely something to refactor...
            Error::Identifier(ref s) => syn::Error::new(
                quote::__private::Span::call_site(),
                format!(
                    "expected `[{}]`, got {}",
                    Identifier::str_list().join(", "),
                    s.as_ref().unwrap_or(&"None".to_string())
                ),
            ),
            Error::Data(input) => {
                syn::Error::new_spanned(input, format!("expected Enum, got {:#?}", input.data))
            }
        }
    }
}
