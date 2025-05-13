
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
