# parsql-macros

A procedural macro crate for Parsql. This package contains derive macros for SQL query generation and parameter handling.

## Features

- Automatic SQL query generation
- Secure parameter management
- Support for multiple database systems (PostgreSQL, SQLite)
- Type-safe database operations
- Automatic protection against SQL Injection attacks

## Macros

- `Updateable`: Generates UPDATE queries
- `Insertable`: Generates INSERT queries
- `Queryable`: Generates SELECT queries
- `Deletable`: Generates DELETE queries
- `SqlParams`: Generates parameter handling code
- `UpdateParams`: Generates parameter handling code for UPDATE operations
- `FromRow`: Generates code for converting database rows to Rust structs

## Installation

Add to your Cargo.toml file as follows:

```toml
[dependencies]
parsql-macros = "0.3.2"
```

and select the feature you need:

```toml
# For SQLite
parsql-macros = { version = "0.3.2", features = ["sqlite"] }

# For PostgreSQL
parsql-macros = { version = "0.3.2", features = ["postgres"] }

# For Tokio PostgreSQL
parsql-macros = { version = "0.3.2", features = ["tokio-postgres"] }

# For Deadpool PostgreSQL
parsql-macros = { version = "0.3.2", features = ["deadpool-postgres"] }
```

## Security Features

### SQL Injection Protection

parsql-macros is designed to be secure against SQL Injection attacks:

- All macros use parameters instead of directly including user data in SQL queries
- WHERE conditions and other SQL components are securely parameterized
- Appropriate parameter placeholders (`$1`, `?`, etc.) are automatically generated for each database adapter
- Parameter evaluation order is preserved to ensure query consistency
- Special character escaping and SQL injection attacks are automatically prevented

## Usage Examples

### Using `Queryable`

```rust
#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[where_clause("id = $")]
pub struct GetUser {
    pub id: i64,
    pub name: String,
    pub email: String,
}

// "SELECT id, name, email FROM users WHERE id = ?" is automatically generated
// and the "id" parameter is securely placed
```

### Using `Insertable`

```rust
#[derive(Insertable)]
#[table("users")]
pub struct InsertUser {
    pub name: String,
    pub email: String,
    pub status: i32,
}

// "INSERT INTO users (name, email, status) VALUES (?, ?, ?)" is automatically generated
// and all fields are securely added as parameters
```

### Using `Updateable`

```rust
#[derive(Updateable, UpdateParams)]
#[table("users")]
#[update("name, email")]
#[where_clause("id = $")]
pub struct UpdateUser {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub status: i32,
}

// "UPDATE users SET name = ?, email = ? WHERE id = ?" is automatically generated
// and values are securely placed
```

### Using `Deletable`

```rust
#[derive(Deletable, SqlParams)]
#[table("users")]
#[where_clause("id = $")]
pub struct DeleteUser {
    pub id: i64,
}

// "DELETE FROM users WHERE id = ?" is automatically generated
// and the "id" parameter is securely placed
```

## Attributes

- `#[table("table_name")]` - Specifies the table name for the query
- `#[where_clause("condition")]` - Defines the WHERE condition ($ sign indicates parameter placement)
- `#[select("field1, field2")]` - Specifies which fields to select for SELECT queries
- `#[update("field1, field2")]` - Specifies which fields to update for UPDATE queries
- `#[join("LEFT JOIN table2 ON table1.id = table2.id")]` - Specifies JOIN statements
- `#[group_by("field1")]` - Specifies GROUP BY statement
- `#[order_by("field1 DESC")]` - Specifies ORDER BY statement
- `#[having("COUNT(*) > 5")]` - Specifies HAVING statement

## Parameter Marking

For each database, appropriate parameter marking is done automatically:

- SQLite: Uses the `?` sign
- PostgreSQL: Uses numbered parameters like `$1, $2, $3, ...`

## License

[MIT license](../LICENSE) 