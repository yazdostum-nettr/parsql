use proc_macro::TokenStream;
use quote::quote;
use regex::Regex;
use syn::{parse_macro_input, Data, DeriveInput, Fields};
use std::env;

/// Extracts field names from a WHERE clause.
/// 
/// # Arguments
/// * `input` - The WHERE clause string
/// 
/// # Returns
/// * `Vec<String>` - A vector of field names found in the WHERE clause
fn extract_fields_from_where_clause(input: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let re = Regex::new(r"\b(\w+)\s*=\s*\$").unwrap();
    for cap in re.captures_iter(input) {
        if let Some(field) = cap.get(1) {
            fields.push(field.as_str().to_string());
        }
    }
    fields
}

/// Query builder module for safe SQL query construction
mod query_builder {
    /// A safe query builder that prevents SQL injection
    #[derive(Default)]
    pub(crate) struct SafeQueryBuilder {
        /// The SQL query being built
        pub query: String,
    }

    impl SafeQueryBuilder {
        /// Creates a new empty query builder
        pub fn new() -> Self {
            Self::default()
        }

        /// Builds and returns the final SQL query string
        pub fn build(self) -> String {
            self.query.trim().to_string()
        }

        /// Adds a SQL keyword to the query with proper spacing
        pub fn add_keyword(&mut self, keyword: &str) {
            if !self.query.is_empty() {
                self.query.push(' ');
            }
            self.query.push_str(keyword);
        }

        /// Adds a safe identifier (table name, column name) to the query
        /// 
        /// # Arguments
        /// * `ident` - The identifier to add
        pub fn add_identifier(&mut self, ident: &str) {
            if !self.query.is_empty() {
                self.query.push(' ');
            }
            let safe_ident = ident
                .chars()
                .filter(|c| c.is_alphanumeric() || *c == '_')
                .collect::<String>();
            self.query.push_str(&safe_ident);
        }

        /// Adds a comma-separated list of safe identifiers to the query
        /// 
        /// # Arguments
        /// * `items` - The list of identifiers to add
        pub fn add_comma_list(&mut self, items: &[&str]) {
            let safe_items: Vec<String> = items
                .iter()
                .map(|item| {
                    item.chars()
                        .filter(|c| c.is_alphanumeric() || *c == '_')
                        .collect()
                })
                .collect();
            self.query.push_str(&safe_items.join(", "));
        }

        /// Adds raw text to the query with proper spacing
        /// This should only be used for trusted input or pre-validated strings
        /// 
        /// # Arguments
        /// * `text` - The raw text to add
        pub fn add_raw(&mut self, text: &str) {
            if !self.query.is_empty() && !text.starts_with(',') {
                self.query.push(' ');
            }
            self.query.push_str(text);
        }
    }
}

/// SQL sorgularındaki parametre sayılarını takip etmek için yardımcı veri yapısı.
/// Bu yapı sayesinde, generate edilen SQL ile SQL parametreleri her zaman senkronize olur.
pub(crate) struct SqlParamCounter {
    /// Şu anki parametre numarası (1'den başlar)
    current: usize,
}

impl SqlParamCounter {
    /// 1'den başlayan yeni bir sayaç oluşturur
    pub fn new() -> Self {
        Self { current: 1 }
    }
    
    /// Mevcut parametre numarasını döndürür ve sayacı bir artırır
    pub fn next(&mut self) -> usize {
        let current = self.current;
        self.current += 1;
        current
    }
    
    /// Mevcut parametre numarasını döndürür (artırmadan)
    pub fn current(&self) -> usize {
        self.current
    }
    
    /// Toplam parametre sayısını döndürür (current - 1)
    pub fn count(&self) -> usize {
        self.current - 1
    }
}

/// WHERE koşulundaki parametre numaralarını doğru şekilde atayan yardımcı fonksiyon.
/// Bu fonksiyon, bağımsız olarak kullanılabilir ve sayaç değerini dışarıdan alır.
pub(crate) fn number_where_clause_params(clause: &str, counter: &mut SqlParamCounter) -> String {
    clause.chars()
        .map(|c| {
            if c == '$' {
                // $ işaretinden sonra numara ekle
                let param_num = counter.next();
                format!("${}", param_num)
            } else {
                // Diğer karakterleri olduğu gibi bırak
                c.to_string()
            }
        })
        .collect::<String>()
}

/// Log mesajlarını yazdırmak için yardımcı fonksiyon
fn log_message(message: &str) {
    if let Ok(trace) = env::var("PARSQL_TRACE") {
        if trace == "1" {
            println!("{}", message);
        }
    }
}

/// Implements the Updateable derive macro.
pub(crate) fn derive_updateable_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Extract table attribute
    let table = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("table"))
        .expect("Missing `#[table = \"...\"]` attribute")
        .parse_args::<syn::LitStr>()
        .expect("Expected a string literal for `table`")
        .value();

    // Extract columns attribute
    let columns_attr = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("update"))
        .expect("Missing `#[update = \"...\"]` attribute")
        .parse_args::<syn::LitStr>()
        .expect("Expected a string literal for `columns`")
        .value();

    let column_order: Vec<String> = columns_attr
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    // Get the optional where_clause attribute
    let where_clause = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("where_clause"))
        .map(|attr| {
            attr.parse_args::<syn::LitStr>()
                .expect("Expected a string literal for where_clause")
                .value()
        });

    // Collect fields from the struct
    let fields = if let syn::Data::Struct(data) = &input.data {
        if let syn::Fields::Named(fields) = &data.fields {
            fields
                .named
                .iter()
                .map(|f| f.ident.as_ref().unwrap().to_string())
                .collect::<Vec<_>>()
        } else {
            panic!("Updateable can only be derived for structs with named fields");
        }
    } else {
        panic!("Updateable can only be derived for structs");
    };

    // Sort fields for `updated_columns`
    let sorted_fields: Vec<_> = column_order
        .iter()
        .filter_map(|col| fields.iter().find(|field| *field == col))
        .cloned()
        .collect();

    // SQL parametrelerinin numaralandırması için SqlParamCounter kullanıyoruz
    let mut param_counter = SqlParamCounter::new();
    
    // SET deyiminde kullanılan parametreler sayacı başlatır (1, 2, ...)
    // Her update edilen alan için bir parametre kullanılır
    for _ in 0..sorted_fields.len() {
        param_counter.next();
    }
    
    // Parametre sayacı update alanlarından sonra devam eder
    // WHERE cümlesindeki parametreler SET parametrelerinden sonraki değerleri alır
    let adjusted_where_clause = where_clause
        .map(|clause| number_where_clause_params(&clause, &mut param_counter))
        .unwrap_or_else(|| "".to_string());

    let mut builder = query_builder::SafeQueryBuilder::new();
    
    builder.add_keyword("UPDATE");
    builder.add_identifier(&table);
    builder.add_keyword("SET");

    // Build SET statements safely
    let update_statements: Vec<String> = column_order
        .iter()
        .enumerate()
        .map(|(i, col)| {
            let safe_col = col.chars()
                .filter(|c| c.is_alphanumeric() || *c == '_')
                .collect::<String>();
            format!("{} = ${}", safe_col, i + 1)
        })
        .collect();
    
    builder.add_raw(&update_statements.join(", "));
    
    if !adjusted_where_clause.is_empty() {
        builder.add_keyword("WHERE");
        builder.add_raw(&adjusted_where_clause);
    }

    let safe_query = builder.build();
    
    // Log mesajlarını PARSQL_TRACE kontrolü ile yazdır
    log_message(&format!("Generated UPDATE SQL: {}", safe_query));
    log_message(&format!("Total param count: {}", param_counter.count()));

    let expanded = quote! {
        impl SqlQuery for #struct_name {
            fn query() -> String {
                #safe_query.to_string()
            }
        }
    };

    TokenStream::from(expanded)
}

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

    // Create placeholders as Vec<String>
    let placeholders: Vec<String> = (1..=column_names.len())
        .map(|i| format!("${}", i))
        .collect();

    let mut builder = query_builder::SafeQueryBuilder::new();
    
    builder.add_keyword("INSERT INTO");
    builder.add_identifier(&table);
    builder.add_keyword("(");
    builder.add_comma_list(&column_names);
    builder.add_keyword(")");
    builder.add_keyword("VALUES");
    builder.add_keyword("(");
    
    let placeholder_str = placeholders.join(", ");
    builder.query.push_str(&placeholder_str);
    
    builder.add_keyword(")");

    let safe_query = builder.build();

    let expanded = quote! {
        impl SqlQuery for #struct_name {
            fn query() -> String {
                #safe_query.to_string()
            }
        }
    };

    TokenStream::from(expanded)
}

pub fn derive_queryable_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Table name and column extraction
    let table = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("table"))
        .expect("Missing `#[table = \"...\"]` attribute")
        .parse_args::<syn::LitStr>()
        .expect("Expected a string literal for table name")
        .value();

    // Get the optional where_clause attribute
    let where_clause = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("where_clause"))
        .map(|attr| {
            attr.parse_args::<syn::LitStr>()
                .expect("Expected a string literal for where_clause")
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
            panic!("Queryable can only be derived for structs with named fields");
        }
    } else {
        panic!("Queryable can only be derived for structs");
    };

    let joins: Vec<String> = input
        .attrs
        .iter()
        .filter(|attr| attr.path().is_ident("join"))
        .map(|attr| {
            attr.parse_args::<syn::LitStr>()
                .expect("Expected a string literal for join")
                .value()
        })
        .collect();

    let tables = table.to_string();

    // SQL parametrelerinin numaralandırması için SqlParamCounter kullanıyoruz
    // Bu sayede tüm parametreler her zaman 1'den başlayacak ve tutarlı şekilde artacak
    let mut param_counter = SqlParamCounter::new();

    // WHERE cümlesini numaralandır
    let adjusted_where_clause = where_clause
        .map(|clause| number_where_clause_params(&clause, &mut param_counter))
        .unwrap_or_else(|| "".to_string());

    // Get the optional select attribute
    let select = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("select"))
        .map(|attr| {
            attr.parse_args::<syn::LitStr>()
                .expect("Expected a string literal for select")
                .value()
        });

    // If select is not defined, use all fields
    let select = select.unwrap_or_else(|| {
        fields
            .iter()
            .map(|f| f.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    });

    // Get the optional group_by attribute
    let group_by = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("group_by"))
        .map(|attr| {
            attr.parse_args::<syn::LitStr>()
                .expect("Expected a string literal for group_by")
                .value()
        });

    // Get the optional having attribute
    let having = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("having"))
        .map(|attr| {
            attr.parse_args::<syn::LitStr>()
                .expect("Expected a string literal for having")
                .value()
        });

    // HAVING cümlesi para counter'ın mevcut değerinden devam eder
    // Böylece WHERE cümlesindeki son parametreden sonraki parametreler kullanılır
    let adjusted_having_clause = having
        .as_ref()
        .map(|clause| number_where_clause_params(clause, &mut param_counter))
        .unwrap_or_else(|| "".to_string());

    // Get the optional order_by attribute
    let order_by = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("order_by"))
        .map(|attr| {
            attr.parse_args::<syn::LitStr>()
                .expect("Expected a string literal for order_by")
                .value()
        });

    let mut builder = query_builder::SafeQueryBuilder::new();
    
    builder.add_keyword("SELECT");
    builder.add_raw(&select);
    builder.add_keyword("FROM");
    builder.add_identifier(&tables);
    
    // Add join expressions separately and place a space around each one
    for join in joins {
        builder.add_raw(&format!(" {} ", join.trim()));
    }
    
    if !adjusted_where_clause.is_empty() {
        builder.add_keyword("WHERE");
        builder.add_raw(&adjusted_where_clause);
    }

    // Add GROUP BY clause
    if let Some(group_by_clause) = group_by {
        builder.add_keyword("GROUP BY");
        builder.add_raw(&group_by_clause);
    }

    // HAVING cümlesi
    if let Some(_) = having {
        builder.add_keyword("HAVING");
        builder.add_raw(&adjusted_having_clause);
    }

    // Add ORDER BY clause
    if let Some(order_by_clause) = order_by {
        builder.add_keyword("ORDER BY");
        builder.add_raw(&order_by_clause);
    }

    // Add LIMIT clause
    let limit = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("limit"))
        .map(|attr| {
            attr.parse_args::<syn::LitInt>()
                .expect("Expected an integer literal for limit")
                .base10_parse::<u64>()
                .expect("Failed to parse limit value as an integer")
        });

    if let Some(limit_value) = limit {
        builder.add_keyword("LIMIT");
        builder.add_raw(&limit_value.to_string());
    }

    // Add OFFSET clause
    let offset = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("offset"))
        .map(|attr| {
            attr.parse_args::<syn::LitInt>()
                .expect("Expected an integer literal for offset")
                .base10_parse::<u64>()
                .expect("Failed to parse offset value as an integer")
        });

    if let Some(offset_value) = offset {
        builder.add_keyword("OFFSET");
        builder.add_raw(&offset_value.to_string());
    }

    let safe_query = builder.build();

    // Log mesajlarını PARSQL_TRACE kontrolü ile yazdır
    log_message(&format!("Generated SQL Query: {}", safe_query));
    log_message(&format!("Total param count: {}", param_counter.count()));

    let expanded = quote! {
        impl SqlQuery for #struct_name {
            fn query() -> String {
                #safe_query.to_string()
            }
        }
    };

    TokenStream::from(expanded)
}

pub(crate) fn derive_deletable_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let table = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("table"))
        .expect("Missing `#[table = \"...\"]` attribute")
        .parse_args::<syn::LitStr>()
        .expect("Expected a string literal for table name")
        .value();

    // Get the optional where_clause attribute
    let where_clause = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("where_clause"))
        .map(|attr| {
            attr.parse_args::<syn::LitStr>()
                .expect("Expected a string literal for where_clause")
                .value()
        });

    // SQL parametrelerinin numaralandırması için SqlParamCounter kullanıyoruz
    // Her zaman 1'den başlar
    let mut param_counter = SqlParamCounter::new();
    
    let adjusted_where_clause = where_clause
        .map(|clause| number_where_clause_params(&clause, &mut param_counter))
        .unwrap_or_else(|| "".to_string());

    let mut builder = query_builder::SafeQueryBuilder::new();
    
    builder.add_keyword("DELETE FROM");
    builder.add_identifier(&table);
    builder.add_keyword("WHERE");
    builder.add_raw(&adjusted_where_clause);  // SafeQueryBuilder will automatically add spaces

    let safe_query = builder.build();
    
    // Log mesajlarını PARSQL_TRACE kontrolü ile yazdır
    log_message(&format!("Generated DELETE SQL: {}", safe_query));
    log_message(&format!("Total param count: {}", param_counter.count()));

    let expanded = quote! {
        impl SqlQuery for #struct_name {
            fn query() -> String {
                #safe_query.to_string()
            }
        }
    };

    TokenStream::from(expanded)
}

pub(crate) fn derive_sql_params_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Get the optional where_clause attribute
    let where_clause = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("where_clause"))
        .map(|attr| {
            attr.parse_args::<syn::LitStr>()
                .expect("Expected a string literal for where_clause")
                .value()
        });

    // HAVING cümlesi için de parametreleri kontrol et
    let having_clause = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("having"))
        .map(|attr| {
            attr.parse_args::<syn::LitStr>()
                .expect("Expected a string literal for having")
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
            panic!("SqlParams can only be derived for structs with named fields");
        }
    } else {
        panic!("SqlParams can only be derived for structs");
    };

    // where_clause ve having_clause'daki parametreleri belirle
    let mut param_fields = Vec::new();
    
    // WHERE cümlesindeki alan adlarını bulma
    if let Some(clause) = &where_clause {
        let where_fields: Vec<_> = fields
            .iter()
            .filter(|&f| clause.contains(f))
            .cloned()
            .collect();
        param_fields.extend(where_fields);
    }
    
    // HAVING cümlesindeki alan adlarını bulma
    if let Some(clause) = &having_clause {
        let having_fields: Vec<_> = fields
            .iter()
            .filter(|&f| clause.contains(f))
            .cloned()
            .collect();
        param_fields.extend(having_fields);
    }
    
    // Eğer hiçbir cümlede parametre yoksa, tüm alanları kullan
    if param_fields.is_empty() {
        param_fields = fields;
    }

    let field_names: Vec<_> = param_fields
        .iter()
        .map(|f| syn::Ident::new(f, struct_name.span()))
        .collect();

    let expanded = quote! {
        impl SqlParams for #struct_name {
            fn params(&self) -> Vec<&(dyn ToSql + Sync)> {
                vec![#(&self.#field_names as &(dyn ToSql + Sync)),*]
            }
        }
    };

    TokenStream::from(expanded)
}

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
    let update_fields: Vec<String> = update
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

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

/// Implements the FromRow trait for SQLite database
/// 
/// # Arguments
/// * `input` - TokenStream containing the struct definition
/// 
/// # Returns
/// * `TokenStream` - Generated implementation code
#[cfg(feature = "sqlite")]
pub(crate) fn derive_from_row_sqlite(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
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

    let expanded = quote! {
        impl FromRow for #name {
            fn from_row(row: &Row) -> Result<Self, Error> {
                Ok(Self {
                    #(#field_names: row.get(#field_strings)?),*
                })
            }
        }
    };

    TokenStream::from(expanded)
}

/// Implements the FromRow trait for PostgreSQL database
/// 
/// # Arguments
/// * `input` - TokenStream containing the struct definition
/// 
/// # Returns
/// * `TokenStream` - Generated implementation code
pub(crate) fn derive_from_row_postgres(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
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

    let gen = quote! {
        impl FromRow for #name {
            fn from_row(row: &Row) -> Result<Self, Error> {
                Ok(Self {
                    #(#field_names: row.try_get(#field_names_str)?),*
                })
            }
        }
    };
    gen.into()
}

