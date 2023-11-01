use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    token, DataStruct, DataUnion, Expr, Fields, Ident, Result, Token, Visibility,
};

use super::NestedDerive;

#[derive(Debug, Clone)]
pub enum NestData {
    Struct(DataStruct),
    Enum(NestDataEnum),
    Union(DataUnion),
}
#[derive(Debug, Clone)]
pub struct NestDataEnum {
    pub enum_token: Token![enum],
    pub brace_token: token::Brace,
    pub variants: Punctuated<NestVariant, Token![,]>,
}

#[derive(Debug, Clone)]
pub struct NestVariant {
    pub attrs: Vec<syn::Attribute>,

    /// Name of the variant.
    pub ident: Ident,

    /// Content stored in the variant.
    pub fields: Fields,

    /// Explicit discriminant: `Variant = 1`
    pub discriminant: Option<(Token![=], Expr)>,

    pub nested: Option<NestedDerive>,
}

impl NestVariant {
    pub fn set_nested(&self, tokens: &mut TokenStream) {
        if let Some(nest) = &self.nested {
            tokens.append_all(nest.to_token_stream());
        }
    }
}

impl Parse for NestVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let _visibility: Visibility = input.parse()?;
        let ident: Ident = input.parse()?;
        let mut nested = None;
        let fields = if input.peek(token::Brace) {
            Fields::Named(input.parse()?)
        } else if input.peek(token::Paren) {
            let content;
            parenthesized!(content in input);
            // panic!("content: {content:#?}");
            if let Ok(derive) = content.parse::<NestedDerive>() {
                let ident = derive.ident.clone();
                nested = Some(derive);
                // panic!("{nested:#?}");
                Fields::Unnamed(parse_quote!((#ident)))
            } else {
                Fields::Unnamed(input.parse()?)
            }
        } else {
            Fields::Unit
        };
        let discriminant = if input.peek(Token![=]) {
            let eq_token: Token![=] = input.parse()?;
            let discriminant: Expr = input.parse()?;
            Some((eq_token, discriminant))
        } else {
            None
        };
        Ok(NestVariant {
            attrs,
            ident,
            fields,
            discriminant,
            nested,
        })
    }
}

impl ToTokens for NestVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.attrs);
        self.ident.to_tokens(tokens);
        self.fields.to_tokens(tokens);
        if let Some((eq_token, disc)) = &self.discriminant {
            eq_token.to_tokens(tokens);
            disc.to_tokens(tokens);
        }
    }
}
