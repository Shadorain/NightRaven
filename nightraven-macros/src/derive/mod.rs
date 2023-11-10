mod attr;
mod error;
mod ident;
mod variant;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    DeriveInput, Generics, Ident, Result,
};

use attr::Attributes;
use error::Error;
use ident::Identifier;
use variant::Variants;

/// NightRavenDerive type.
///
/// Houses the entire parsed macro contents.
///
/// * `name`: Main enum identifier.
/// * `generics`: Any generic information.
/// * `attrs`: Base level attributes.
/// * `variants`: All variants.
#[derive(Debug)]
pub struct NightRavenDerive {
    name: Ident,
    generics: Generics,
    attrs: Attributes,
    variants: Variants,
}

/// Creates an `NightRavenDerive` type from parsed macro input.
///
/// * `input`: Parsed macro input.
impl Parse for NightRavenDerive {
    fn parse(input: ParseStream) -> Result<Self> {
        let input = input.parse::<DeriveInput>()?;
        let variants = Variants::from(input.clone())?;
        Ok(Self {
            name: input.ident,
            generics: input.generics,
            attrs: Attributes::from(&input.attrs)?,
            variants,
        })
    }
}

/// Builds the entire macro output.
impl ToTokens for NightRavenDerive {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let name = &self.name;
        let getters = self.variants.build_getters(&self.attrs);
        let arms = self.variants.build_arms();
        quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                #(#getters)*
            }

            impl #impl_generics NightRaven for #name #ty_generics #where_clause {
                fn list_names(&self) -> &'static [&'static str] {
                    todo!();
                    // match self {
                    //     #(#arms),*
                    // }
                }
                // fn concatenated_names(&self) -> Option<&'static str> {
                //     match self {
                //         #(#arms),*
                //     }
                // }
            }
        }
        .to_tokens(tokens);
    }
}
