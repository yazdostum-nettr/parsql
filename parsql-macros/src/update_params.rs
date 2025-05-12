use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

use crate::{
    extract_fields_from_where_clause, log_message, number_where_clause_params, query_builder,
    SqlParamCounter,
};


pub(crate) fn derive_update_params_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Get the update attribute
    let update = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("update"))
        .expect("Missing `#[update = \"...\"]` attribute")
        .parse_args::<syn::LitStr>()
        .expect("Expected a string literal for update")
        .value();

    // Get the where_clause attribute
    let where_clause = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("where_clause"))
        .expect("Missing `#[where_clause = \"...\"]` attribute")
        .parse_args::<syn::LitStr>()
        .expect("Expected a string literal for where_clause")
        .value();

    let fields = if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            fields
                .named
                .iter()
                .map(|f| f.ident.as_ref().unwrap().to_string())
                .collect::<Vec<_>>()
        } else {
            panic!("UpdateParams can only be derived for structs with named fields");
        }
    } else {
        panic!("UpdateParams can only be derived for structs");
    };

    // Get fields to be used for update
    let update_fields: Vec<String> = update.split(',').map(|s| s.trim().to_string()).collect();

    // Get fields to be used in the where clause
    let condition_fields = extract_fields_from_where_clause(&where_clause);

    // Create field names
    let update_field_names: Vec<_> = update_fields
        .iter()
        .filter_map(|col| fields.iter().find(|field| *field == col))
        .map(|f| syn::Ident::new(f, struct_name.span()))
        .collect();

    let condition_field_names: Vec<_> = condition_fields
        .iter()
        .filter_map(|col| fields.iter().find(|field| *field == col))
        .map(|f| syn::Ident::new(f, struct_name.span()))
        .collect();

    let expanded = quote! {
        impl UpdateParams for #struct_name {
            fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
                let update_values: Vec<&(dyn ToSql + Sync)> = vec![#(&self.#update_field_names as &(dyn ToSql + Sync)),*];
                let condition_values: Vec<&(dyn ToSql + Sync)> = vec![#(&self.#condition_field_names as &(dyn ToSql + Sync)),*];

                [update_values, condition_values].concat()
            }
        }
    };

    TokenStream::from(expanded)
}