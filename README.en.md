# parsql
An experimental SQL helper library

## What It Does

Parsql is a library that allows you to manage SQL queries directly through Rust structs. Its main purpose is to make database operations safer and with less code. With this library, you can:

- Generate automatic SQL queries from struct definitions
- Manage database parameters safely
- Easily perform generic CRUD operations (create, read, update, delete)
- Create dynamic SQL and execute complex queries
- Easily perform asynchronous database operations
- Get automatic protection against SQL injection attacks

Parsql is not a standard ORM. Instead, it focuses on simplifying SQL writing and usage.

## Supported Databases

Parsql supports the following database systems:

- **SQLite** (synchronous): `parsql-sqlite` package
- **PostgreSQL** (synchronous): `parsql-postgres` package
- **Tokio PostgreSQL** (asynchronous): `parsql-tokio-postgres` package
- **Deadpool PostgreSQL** (asynchronous connection pool): `parsql-deadpool-postgres` package

## Installation

When adding the crate to your application, you need to specify which database you'll be working with as a 'feature'. You can add the package to your Cargo.toml file as follows:

### For SQLite
```toml
parsql = { version = "0.3.2", features = ["sqlite"] }
```

### For PostgreSQL
```toml
parsql = { version = "0.3.2", features = ["postgres"] }
```

### For Tokio PostgreSQL
```toml
parsql = { version = "0.3.2", features = ["tokio-postgres"] }
```

### For Deadpool PostgreSQL connection pool
```toml
parsql = { version = "0.3.2", features = ["deadpool-postgres"] }
```

## Core Features

### Procedural Macros
Parsql offers various procedural macros to facilitate database operations:

- `#[derive(Queryable)]` - For read (select) operations
- `#[derive(Insertable)]` - For insert operations
- `#[derive(Updateable)]` - For update operations
- `#[derive(Deletable)]` - For delete operations
- `#[derive(FromRow)]` - For converting database results to objects
- `#[derive(SqlParams)]` - For configuring SQL parameters
- `#[derive(UpdateParams)]` - For configuring update parameters

### Security Features

#### SQL Injection Protection
Parsql is designed to be secure against SQL injection attacks:

- Parameterized queries are automatically used, never direct string concatenation
- All user inputs are safely parameterized
- Macros process SQL parameters correctly and provide a secure format
- Appropriate parameter placeholders (`$1`, `?`, etc.) are automatically applied for each database adapter
- The need for manual string concatenation when writing SQL is eliminated
- Security measures are fully maintained even in asynchronous contexts

```rust
// Example of secure parameter usage
#[derive(Queryable, FromRow, SqlParams)]
#[table("users")]
#[where_clause("username = $ AND status = $")]
struct UserQuery {
    username: String,
    status: i32,
}

// Parameters are securely placed,
// no risk of SQL injection
let query = UserQuery {
    username: user_input,
    status: 1,
};
```

### Attributes
You can use various attributes to customize your queries:

- `#[table("table_name")]` - To specify the table name
- `#[where_clause("id = $")]` - To specify the WHERE condition
- `#[select("field1, field2")]` - To customize the SELECT statement
- `#[update("field1, field2")]` - To customize the UPDATE statement
- `#[join("LEFT JOIN table2 ON table1.id = table2.fk_id")]` - For JOIN statements
- `#[group_by("field1")]` - For GROUP BY statements
- `#[order_by("field1 DESC")]` - For ORDER BY statements
- `#[having("COUNT(*) > 5")]` - For HAVING statements
- `#[limit(10)]` - For LIMIT statements
- `#[offset(5)]` - For OFFSET statements
- `#[returning("id")]` - To specify returning values from INSERT/UPDATE operations

### SQL Tracing
To monitor SQL queries generated during development:

```sh
PARSQL_TRACE=1 cargo run
```

This will print all executed SQL queries to the console.

## Simple Usage Examples

### Using with SQLite

```rust
use parsql::{
    sqlite::{get, insert},
    macros::{Queryable, Insertable, FromRow, SqlParams},
};
use rusqlite::Connection;

// To retrieve a record
#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[where_clause("id = $")]
pub struct GetUser {
    pub id: i64,
    pub name: String,
    pub email: String,
}

impl GetUser {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            name: Default::default(),
            email: Default::default(),
        }
    }
}

// To insert a new record
#[derive(Insertable, SqlParams)]
#[table("users")]
pub struct InsertUser {
    pub name: String,
    pub email: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("test.db")?;
    
    let insert_user = InsertUser {
        name: "John".to_string(),
        email: "john@example.com".to_string(),
    };
    
    let id = insert(&conn, insert_user)?;
    println!("Inserted record ID: {}", id);
    
    let get_user = GetUser::new(id);
    let user = get(&conn, get_user)?;
    println!("User: {:?}", user);
    
    Ok(())
}
```

### Asynchronous Usage with Tokio-Postgres

```rust
use parsql::{
    tokio_postgres::{get, insert},
    macros::{Queryable, Insertable, FromRow, SqlParams},
};
use tokio_postgres::{NoTls, Error};

#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[where_clause("id = $")]
pub struct GetUser {
    pub id: i64,
    pub name: String,
    pub email: String,
}

impl GetUser {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            name: Default::default(),
            email: Default::default(),
        }
    }
}

#[derive(Insertable, SqlParams)]
#[table("users")]
pub struct InsertUser {
    pub name: String,
    pub email: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres dbname=test",
        NoTls,
    ).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
    
    let insert_user = InsertUser {
        name: "John".to_string(),
        email: "john@example.com".to_string(),
    };
    
    let id = insert(&client, insert_user).await?;
    println!("Inserted record ID: {}", id);
    
    let get_user = GetUser::new(id);
    let user = get(&client, get_user).await?;
    println!("User: {:?}", user);
    
    Ok(())
}
```

## Performance Tips

- Reuse queries with the same SQL structure to take advantage of the query plan cache
- Use connection pools for database-intensive applications
- Use pagination (limit and offset) instead of `get_all` for large datasets
- Apply filters at the database level, not in your application

## Detailed Documentation

For more detailed information and examples for each database adapter, refer to the README files in the respective sub-packages:

- [SQLite Documentation](./parsql-sqlite/README.en.md)
- [PostgreSQL Documentation](./parsql-postgres/README.en.md)
- [Tokio PostgreSQL Documentation](./parsql-tokio-postgres/README.en.md)
- [Deadpool PostgreSQL Documentation](./parsql-deadpool-postgres/README.en.md)

You can find comprehensive example projects for each database type in the [examples folder](./examples) on GitHub.

## Changes in Version 0.3.0

- Added `join`, `group_by`, `order_by`, and `having` attributes
- Added `PARSQL_TRACE` environment variable support
- Updated attribute names (`table_name`→`table`, `update_clause`→`update`, `select_clause`→`select`)
- Added `SqlQuery` trait and simplified trait structure
- `deadpool-postgres`, which was available as a feature in the `parsql-tokio-postgres` package, has been refactored into the `parsql-deadpool-postgres` package

## Licensing

This library is licensed under the MIT or Apache-2.0 license.