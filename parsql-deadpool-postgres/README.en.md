# parsql-deadpool-postgres

Deadpool PostgreSQL integration crate for parsql. This package provides APIs that support asynchronous connection pool management with the deadpool-postgres library.

## Features

- PostgreSQL connection pool management with Deadpool
- Asynchronous PostgreSQL operations (with tokio runtime)
- Automatic SQL query generation
- Secure parameter management
- Generic CRUD operations (get, insert, update, delete)
- Conversion of database rows to structs
- Custom row transformations
- Automatic protection against SQL Injection attacks
- Transaction support

## Security Features

### SQL Injection Protection

parsql-deadpool-postgres is designed to be secure against SQL Injection attacks:

- All user inputs are automatically parameterized
- PostgreSQL's "$1, $2, ..." parameterization structure is used automatically
- Macros process SQL parameters securely, providing protection against injection attacks
- Parameters are automatically managed to ensure correct order and type
- User inputs in `#[where_clause]` and other SQL components are always parameterized
- Security measures are fully maintained even when using connection pools

```rust
// SQL injection protection example
#[derive(Queryable)]
#[table("users")]
#[where_clause("username = $ AND status = $")]
struct UserQuery {
    username: String,
    status: i32,
}

// User input (potentially harmful) is used safely
let query = UserQuery {
    username: user_input, // This value is not directly inserted into SQL, it's parameterized
    status: 1,
};

// Generated query: "SELECT * FROM users WHERE username = $1 AND status = $2"
// Parameters are safely sent as: [user_input, 1]
let user = get(&pool, &query).await?;
```

## Installation

Add to your Cargo.toml file as follows:

```toml
[dependencies]
# For Deadpool PostgreSQL
parsql-deadpool-postgres = "0.3.2"
parsql-macros = "0.3.2"
deadpool-postgres = "0.14"
tokio-postgres = "0.7"
tokio = { version = "1", features = ["full"] }
```

## Basic Usage

This package uses **asynchronous operations** and **connection pool management** when working with PostgreSQL databases. This means it requires async/await usage.

### Creating a Connection Pool

```rust
use deadpool_postgres::{Config, Runtime};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create PostgreSQL connection pool
    let mut cfg = Config::new();
    cfg.host = Some("localhost".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("postgres".to_string());
    cfg.dbname = Some("test".to_string());
    
    // Create connection pool
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    
    // ... pool usage ...
    
    Ok(())
}
```

### Defining Models

Define model structs marked with the relevant macros for database CRUD operations:

```rust
use parsql_deadpool_postgres::macros::{Insertable, Updateable, Queryable, Deletable};

// Model for insertion
#[derive(Insertable)]
#[table("users")]
struct UserInsert {
    name: String,
    email: String,
    active: bool,
}

// Model for updating
#[derive(Updateable)]
#[table("users")]
#[update("name, email, active")]
#[where_clause("id = $")]
struct UserUpdate {
    id: i32,
    name: String,
    email: String,
    active: bool,
}

// Model for querying
#[derive(Queryable)]
#[table("users")]
#[select("id, name, email, active")] 
#[where_clause("id = $")]
struct UserById {
    id: i32,
    name: String,
    email: String,
    active: bool,
}

// Model for deletion
#[derive(Deletable)]
#[table("users")]
#[where_clause("id = $")]
struct UserDelete {
    id: i32,
}
```

### CRUD Operations

#### Data Insertion

```rust
use parsql_deadpool_postgres::insert;

let user = UserInsert {
    name: "John Doe".to_string(),
    email: "john@example.com".to_string(),
    active: true,
};

let result = insert(&pool, user).await?;
println!("Number of inserted records: {}", result);
```

#### Data Update

```rust
use parsql_deadpool_postgres::update;

let user = UserUpdate {
    id: 1,
    name: "John Doe (Updated)".to_string(),
    email: "john.updated@example.com".to_string(),
    active: true,
};

let success = update(&pool, user).await?;
println!("Update successful: {}", success);
```

#### Data Querying

```rust
use parsql_deadpool_postgres::{get, get_all};

// Fetch a single record
let query = UserById { id: 1, ..Default::default() };
let user = get(&pool, &query).await?;

// Fetch multiple records
let query = UsersByActive { active: true, ..Default::default() };
let active_users = get_all(&pool, &query).await?;
```

#### Data Deletion

```rust
use parsql_deadpool_postgres::delete;

let user_delete = UserDelete { id: 1 };
let deleted_count = delete(&pool, user_delete).await?;
println!("Number of deleted records: {}", deleted_count);
```

### Custom Row Transformations

To transform query results into a different structure:

```rust
use parsql_deadpool_postgres::select_all;
use tokio_postgres::Row;

// Summary data model
struct UserSummary {
    id: i32,
    full_name: String,
}

// Query with custom transformation function
let query = UsersByActive { active: true, ..Default::default() };

let summaries = select_all(&pool, query, |row: &Row| UserSummary {
    id: row.get("id"),
    full_name: row.get("name"),
}).await?;
```

### Transaction Operations

```rust
// Perform operations with transaction
let client = pool.get().await?;
let tx = client.transaction().await?;

// Operations...
let user = UserInsert { ... };
let (tx, result) = parsql_deadpool_postgres::transactional::tx_insert(tx, user).await?;

// Commit the transaction
tx.commit().await?;
```

## Example Project

For a more comprehensive example, see the `/examples/tokio-deadpool-postgres` directory in the project.

## License

This project is licensed under the MIT License.