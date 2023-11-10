use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token, Attribute, DataStruct, DataUnion, Fields, FieldsNamed, Generics, Ident, Result, Token,
    Visibility, WhereClause,
};

use super::{NestData, NestDataEnum, NestVariant};

#[derive(Debug, Clone)]
pub struct NestedDerive {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub generics: Generics,
    pub data: NestData,
}

impl Parse for NestedDerive {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse::<syn::Visibility>()?;

        let lookahead = input.lookahead1();
        if lookahead.peek(Token![struct]) {
            let struct_token = input.parse::<Token![struct]>()?;
            let ident = input.parse::<Ident>()?;
            let generics = input.parse::<Generics>()?;
            let (where_clause, fields, semi) = Self::data_struct(input)?;
            Ok(Self {
                attrs,
                vis,
                ident,
                generics: Generics {
                    where_clause,
                    ..generics
                },
                data: NestData::Struct(DataStruct {
                    struct_token,
                    fields,
                    semi_token: semi,
                }),
            })
        } else if lookahead.peek(Token![enum]) {
            let enum_token = input.parse::<Token![enum]>()?;
            let ident = input.parse::<Ident>()?;
            let generics = input.parse::<Generics>()?;
            let (where_clause, brace, variants) = Self::data_enum(input)?;
            Ok(Self {
                attrs,
                vis,
                ident,
                generics: Generics {
                    where_clause,
                    ..generics
                },
                data: NestData::Enum(NestDataEnum {
                    enum_token,
                    brace_token: brace,
                    variants,
                }),
            })
        } else if lookahead.peek(Token![union]) {
            let union_token = input.parse::<Token![union]>()?;
            let ident = input.parse::<Ident>()?;
            let generics = input.parse::<Generics>()?;
            let (where_clause, fields) = Self::data_union(input)?;
            Ok(Self {
                attrs,
                vis,
                ident,
                generics: Generics {
                    where_clause,
                    ..generics
                },
                data: NestData::Union(DataUnion {
                    union_token,
                    fields,
                }),
            })
        } else {
            Err(lookahead.error())
        }
    }
}

fn attrs_outer(
    attrs: &[Attribute],
) -> core::iter::Filter<core::slice::Iter<'_, Attribute>, fn(&&Attribute) -> bool> {
    fn is_outer(attr: &&Attribute) -> bool {
        match attr.style {
            syn::AttrStyle::Outer => true,
            syn::AttrStyle::Inner(_) => false,
        }
    }
    attrs.iter().filter(is_outer)
}

struct TokensOrDefault<'a, T: 'a>(pub &'a Option<T>);

impl<'a, T> ToTokens for TokensOrDefault<'a, T>
where
    T: ToTokens + Default,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.0 {
            Some(t) => t.to_tokens(tokens),
            None => T::default().to_tokens(tokens),
        }
    }
}

impl ToTokens for NestedDerive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for attr in attrs_outer(&self.attrs) {
            attr.to_tokens(tokens);
        }
        self.vis.to_tokens(tokens);
        match &self.data {
            NestData::Struct(d) => d.struct_token.to_tokens(tokens),
            NestData::Enum(d) => d.enum_token.to_tokens(tokens),
            NestData::Union(d) => d.union_token.to_tokens(tokens),
        }
        self.ident.to_tokens(tokens);
        self.generics.to_tokens(tokens);
        match &self.data {
            NestData::Struct(data) => match &data.fields {
                Fields::Named(fields) => {
                    self.generics.where_clause.to_tokens(tokens);
                    fields.to_tokens(tokens);
                }
                Fields::Unnamed(fields) => {
                    fields.to_tokens(tokens);
                    self.generics.where_clause.to_tokens(tokens);
                    TokensOrDefault(&data.semi_token).to_tokens(tokens);
                }
                Fields::Unit => {
                    self.generics.where_clause.to_tokens(tokens);
                    TokensOrDefault(&data.semi_token).to_tokens(tokens);
                }
            },
            NestData::Enum(data) => {
                self.generics.where_clause.to_tokens(tokens);
                data.brace_token.surround(tokens, |tokens| {
                    data.variants.to_tokens(tokens);
                });

                for var in data.variants.iter() {
                    var.set_nested(tokens);
                }
            }
            NestData::Union(data) => {
                self.generics.where_clause.to_tokens(tokens);
                data.fields.to_tokens(tokens);
            }
        }
    }
}

impl NestedDerive {
    fn data_struct(input: ParseStream) -> Result<(Option<WhereClause>, Fields, Option<Token![;]>)> {
        let mut lookahead = input.lookahead1();
        let mut where_clause = None;
        if lookahead.peek(Token![where]) {
            where_clause = Some(input.parse()?);
            lookahead = input.lookahead1();
        }

        if where_clause.is_none() && lookahead.peek(token::Paren) {
            let fields = input.parse()?;

            lookahead = input.lookahead1();
            if lookahead.peek(Token![where]) {
                where_clause = Some(input.parse()?);
                lookahead = input.lookahead1();
            }

            if lookahead.peek(Token![;]) {
                let semi = input.parse()?;
                Ok((where_clause, Fields::Unnamed(fields), Some(semi)))
            } else {
                Err(lookahead.error())
            }
        } else if lookahead.peek(token::Brace) {
            let fields = input.parse()?;
            Ok((where_clause, Fields::Named(fields), None))
        } else if lookahead.peek(Token![;]) {
            let semi = input.parse()?;
            Ok((where_clause, Fields::Unit, Some(semi)))
        } else {
            Err(lookahead.error())
        }
    }

    fn data_enum(
        input: ParseStream,
    ) -> Result<(
        Option<WhereClause>,
        token::Brace,
        Punctuated<NestVariant, Token![,]>,
    )> {
        let where_clause = input.parse()?;

        let content;
        let brace = braced!(content in input);
        let variants = content.parse_terminated(NestVariant::parse, Token![,])?;

        Ok((where_clause, brace, variants))
    }

    fn data_union(input: ParseStream) -> Result<(Option<WhereClause>, FieldsNamed)> {
        let where_clause = input.parse()?;
        let fields = input.parse()?;
        Ok((where_clause, fields))
    }
}
