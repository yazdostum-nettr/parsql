use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;
use proc_macro2::TokenStream as TokenStream2;

use crate::implementations;

pub fn expand_from_row(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let pg = implementations::postgres::generate_from_row(&input);
    let sqlite = implementations::sqlite::generate_from_row(&input);

    let tokens: TokenStream2 = quote! {
        #pg
        #sqlite
    };
    TokenStream::from(tokens)
}
