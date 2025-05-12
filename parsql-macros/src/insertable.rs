use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use quote::quote;
use crate::query_builder;

/// Implements the Insertable derive macro.
pub(crate) fn derive_insertable_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Extract table name and columns
    let table = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("table"))
        .expect("Missing `#[table = \"...\"]` attribute")
        .parse_args::<syn::LitStr>()
        .expect("Expected a string literal for table name")
        .value();

    // Extract returning column if specified
    let returning_column = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("returning"))
        .map(|attr| {
            attr.parse_args::<syn::LitStr>()
                .expect("Expected a string literal for returning column")
                .value()
        });

    let fields = if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            fields
                .named
                .iter()
                .map(|f| f.ident.as_ref().unwrap().to_string())
                .collect::<Vec<_>>()
        } else {
            panic!("Insertable can only be derived for structs with named fields");
        }
    } else {
        panic!("Insertable can only be derived for structs");
    };

    let column_names = fields.iter().map(|f| f.as_str()).collect::<Vec<_>>();

    let safe_query = if cfg!(any(feature = "postgres", feature = "tokio-postgres", feature = "deadpool-postgres")) {
        // PostgreSQL için sorgu oluştur
        let mut builder = query_builder::SafeQueryBuilder::new();
        
        builder.add_keyword("INSERT INTO");
        builder.add_identifier(&table);
        builder.add_keyword("(");
        builder.add_comma_list(&column_names);
        builder.add_keyword(")");
        builder.add_keyword("VALUES");
        builder.add_keyword("(");
        
        let placeholders: Vec<String> = (1..=column_names.len())
            .map(|i| format!("${}", i))
            .collect();
        builder.query.push_str(&placeholders.join(", "));
        
        builder.add_keyword(")");

        if let Some(ref column) = returning_column {
            builder.add_keyword("RETURNING");
            builder.add_identifier(column);
        }

        builder.build()
    } else if cfg!(feature = "sqlite") {
        // SQLite için sorgu oluştur
        let mut builder = query_builder::SafeQueryBuilder::new();
        
        builder.add_keyword("INSERT INTO");
        builder.add_identifier(&table);
        builder.add_keyword("(");
        builder.add_comma_list(&column_names);
        builder.add_keyword(")");
        builder.add_keyword("VALUES");
        builder.add_keyword("(");
        
        let placeholders: Vec<String> = (1..=column_names.len())
            .map(|i| format!("?{}", i))
            .collect();
        builder.query.push_str(&placeholders.join(", "));
        
        builder.add_keyword(")");

        if let Some(ref column) = returning_column {
            builder.add_keyword(";");
            builder.add_keyword("SELECT");
            builder.add_keyword("last_insert_rowid()");
            builder.add_keyword("AS");
            builder.add_identifier(column);
        }

        builder.build()
    } else {
        panic!("At least one database feature must be enabled (postgres or sqlite)")
    };

    let expanded = quote! {
        impl SqlQuery for #struct_name {
            fn query() -> String {
                #safe_query.to_string()
            }
        }
    };

    TokenStream::from(expanded)
}
