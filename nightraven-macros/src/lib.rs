use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

use nest::NestedDerive;

mod nest;

#[proc_macro]
pub fn nightraven(input: TokenStream) -> TokenStream {
    let i = input.clone();
    let tokens = parse_macro_input!(i as NestedDerive);
    // eprintln!("Tokens: {tokens:#?}");
    eprintln!("{}", tokens.to_token_stream());

    tokens.to_token_stream().into()
}
