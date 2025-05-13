use quote::quote;
use syn::{Data, DeriveInput, Fields};

/// Implements the FromRow trait for PostgreSQL database
/// 
/// # Arguments
/// * `input` - TokenStream containing the struct definition
/// 
/// # Returns
/// * `TokenStream` - Generated implementation code
pub fn generate_from_row(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    
    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("FromRow only supports structs with named fields"),
        },
        _ => panic!("FromRow only supports structs"),
    };

    let field_names = fields.iter().map(|f| &f.ident);
    let field_names_str = fields.iter().map(|f| f.ident.as_ref().unwrap().to_string());

    quote! {
        impl FromRow for #name {
            fn from_row(row: &Row) -> Result<Self, Error> {
                Ok(Self {
                    #(#field_names: row.try_get(#field_names_str)?),*
                })
            }
        }
    }
}
