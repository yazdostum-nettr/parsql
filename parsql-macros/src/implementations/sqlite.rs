use quote::quote;
use syn::{Data, DeriveInput, Fields};
use proc_macro2::TokenStream;

/// Implements the FromRow trait for SQLite database
/// 
/// # Arguments
/// * `input` - TokenStream containing the struct definition
/// 
/// # Returns
/// * `TokenStream` - Generated implementation code
pub fn generate_from_row(input: &DeriveInput) -> TokenStream {
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => fields,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let field_names = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
    let field_strings = fields
        .named
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string());

    quote! {
        impl FromRow for #name {
            fn from_row(row: &Row) -> Result<Self, Error> {
                Ok(Self {
                    #(#field_names: row.get(#field_strings)?),*
                })
            }
        }
    }
}

