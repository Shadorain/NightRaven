use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

use derive::NightRavenDerive;
use nest::NestedDerive;

mod derive;
mod nest;

/// Provides a derive macro for the `NightRaven` trait.
///
/// ```ignore
/// #[derive(NightRaven)]
/// ```
#[proc_macro_derive(NightRaven, attributes(nightraven))]
pub fn derive_nightraven(input: TokenStream) -> TokenStream {
    let tokens = parse_macro_input!(input as NightRavenDerive);
    eprintln!("{}", tokens.to_token_stream());
    tokens.to_token_stream().into()
}

#[proc_macro]
pub fn nightraven(input: TokenStream) -> TokenStream {
    let tokens = parse_macro_input!(input as NestedDerive);
    eprintln!("{}", tokens.to_token_stream());
    tokens.to_token_stream().into()
}
