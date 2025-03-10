# parsql-tokio-postgres

Tokio PostgreSQL integration crate for parsql. This package provides asynchronous APIs that enable parsql to work with tokio-postgres and deadpool-postgres libraries.

## Features

- Asynchronous PostgreSQL operations (with tokio runtime)
- Automatic SQL query generation
- Secure parameter management
- Generic CRUD operations (get, insert, update, delete)
- Conversion of database rows to structs
- Deadpool connection pool support
- Automatic protection against SQL Injection attacks
- Detailed error reporting
- High-performance asynchronous query execution

## Security Features

### SQL Injection Protection

parsql-tokio-postgres is designed to be secure against SQL Injection attacks:

- All user inputs are automatically parameterized
- PostgreSQL's "$1, $2, ..." parameterization structure is used automatically
- Macros process SQL parameters securely, providing protection against injection attacks
- Parameters are automatically managed to ensure correct order and type
- User inputs in `#[where_clause]` and other SQL components are always parameterized
- Security measures are fully maintained even in asynchronous contexts

```rust
// SQL injection protection example
#[derive(Queryable, FromRow, SqlParams)]
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
let user = get(&client, query).await?;
```

## Installation

Add to your Cargo.toml file as follows:

```toml
[dependencies]
# For Tokio PostgreSQL
parsql = { version = "0.3.2", features = ["tokio-postgres"] }

# or for using with deadpool connection pool
parsql = { version = "0.3.2", features = ["deadpool-postgres"] }
```

or if you want to use this package directly:

```toml
[dependencies]
parsql-tokio-postgres = "0.3.2"
parsql-macros = "0.3.2"
tokio-postgres = "0.7.13"
tokio = { version = "1.41.1", features = ["full"] }

# If you want to use Deadpool
deadpool-postgres = "0.14.1"
```

## Basic Usage

This package uses **asynchronous operations** when working with PostgreSQL databases. This means it requires async/await usage.

### Establishing a Connection

#### With Tokio PostgreSQL

```rust
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create PostgreSQL connection
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=test",
        NoTls,
    ).await?;
    
    // Run the connection in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
    
    // Create example table
    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            state SMALLINT NOT NULL
        )",
        &[],
    ).await?;
    
    // ...
    
    Ok(())
}
```

#### With Deadpool PostgreSQL Connection Pool

```rust
use deadpool_postgres::{Config, Client, Pool};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Deadpool configuration
    let mut cfg = Config::new();
    cfg.host = Some("localhost".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("postgres".to_string());
    cfg.dbname = Some("test".to_string());
    
    // Create connection pool
    let pool = cfg.create_pool(None, NoTls)?;
    
    // Get connection from pool
    let client: Client = pool.get().await?;
    
    // ...
    
    Ok(())
}
```

## CRUD Operations

### Reading Data (Get)

```rust
use parsql::{
    core::Queryable,
    macros::{FromRow, Queryable, SqlParams},
    tokio_postgres::{FromRow, SqlParams, get},
};
use tokio_postgres::{types::ToSql, Row};

#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[where_clause("id = $")]
pub struct GetUser {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

impl GetUser {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            name: Default::default(),
            email: Default::default(),
            state: Default::default(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=test",
        NoTls,
    ).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
    
    // Usage
    let get_user = GetUser::new(1);
    let get_result = get(&client, get_user).await?;
    
    println!("User: {:?}", get_result);
    Ok(())
}
```

### Adding Data (Insert)

```rust
use parsql::{
    core::Insertable,
    macros::{Insertable, SqlParams},
    tokio_postgres::{SqlParams, insert},
};
use tokio_postgres::types::ToSql;

#[derive(Insertable, SqlParams)]
#[table("users")]
pub struct InsertUser {
    pub name: String,
    pub email: String,
    pub state: i16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=test",
        NoTls,
    ).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
    
    let insert_user = InsertUser {
        name: "John".to_string(),
        email: "john@parsql.com".to_string(),
        state: 1,
    };
    
    let insert_result = insert(&client, insert_user).await?;
    println!("Inserted record ID: {}", insert_result);
    
    Ok(())
}
```

### Updating Data (Update)

```rust
use parsql::{
    core::Updateable,
    macros::{UpdateParams, Updateable},
    tokio_postgres::{UpdateParams, update},
};

#[derive(Updateable, UpdateParams)]
#[table("users")]
#[update("name, email")]
#[where_clause("id = $")]
pub struct UpdateUser {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=test",
        NoTls,
    ).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
    
    let update_user = UpdateUser {
        id: 1,
        name: "John Smith".to_string(),
        email: "john.smith@parsql.com".to_string(),
    };
    
    let update_result = update(&client, update_user).await?;
    println!("Update successful: {}", update_result);
    
    Ok(())
}
```

### Deleting Data (Delete)

```rust
use parsql::{
    core::Deletable,
    macros::{Deletable, SqlParams},
    tokio_postgres::{SqlParams, delete},
};

#[derive(Deletable, SqlParams)]
#[table("users")]
#[where_clause("id = $")]
pub struct DeleteUser {
    pub id: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=test",
        NoTls,
    ).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
    
    let delete_user = DeleteUser { id: 1 };
    let delete_result = delete(&client, delete_user).await?;
    println!("Number of records deleted: {}", delete_result);
    
    Ok(())
}
```

## Custom Queries

Sometimes standard CRUD operations might not be sufficient. The `select` and `select_all` functions are provided to easily execute custom queries:

```rust
use parsql::{
    core::Queryable,
    macros::{Queryable, SqlParams},
    tokio_postgres::{SqlParams, select, select_all, FromRow},
};
use tokio_postgres::Row;

#[derive(Queryable, SqlParams)]
#[table("users")]
#[select("SELECT u.*, p.role FROM users u JOIN profiles p ON u.id = p.user_id")]
#[where_clause("u.state = $")]
pub struct UserWithRole {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
    pub role: String,
}

// Manually implementing FromRow trait
impl FromRow for UserWithRole {
    fn from_row(row: &Row) -> Result<Self, tokio_postgres::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            email: row.try_get("email")?,
            state: row.try_get("state")?,
            role: row.try_get("role")?,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=test",
        NoTls,
    ).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
    
    let query = UserWithRole {
        id: 0,
        name: String::new(),
        email: String::new(),
        state: 1, // Active users
        role: String::new(),
    };
    
    // To get a single result
    let user = select(&client, query.clone(), |row| UserWithRole::from_row(row)).await?;
    println!("User: {:?}", user);
    
    // To get all results
    let users = select_all(&client, query, |row| {
        UserWithRole::from_row(row).unwrap()
    }).await?;
    
    println!("User count: {}", users.len());
    
    Ok(())
}
```

## Using with Deadpool Connection Pool

The Deadpool connection pool allows you to efficiently manage connections for a large number of concurrent database operations. To use it, enable the `deadpool-postgres` feature:

```rust
use parsql::{
    tokio_postgres::{get, FromRow, SqlParams},
    macros::{FromRow, Queryable, SqlParams},
};
use deadpool_postgres::{Config, Client, Pool};
use tokio_postgres::NoTls;

#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[where_clause("state = $")]
pub struct ActiveUsers {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

impl ActiveUsers {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
            email: String::new(),
            state: 1, // Active users
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Deadpool configuration
    let mut cfg = Config::new();
    cfg.host = Some("localhost".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("postgres".to_string());
    cfg.dbname = Some("test".to_string());
    
    // Create connection pool
    let pool = cfg.create_pool(None, NoTls)?;
    
    // Get connection from pool
    let client: Client = pool.get().await?;
    
    // Create query
    let query = ActiveUsers::new();
    
    // Get active users
    let active_users = get(&client, query).await?;
    println!("Active user: {:?}", active_users);
    
    Ok(())
}
```

## Advanced Features and Optimizations

### SQL Tracing

For debugging purposes, you can use the `PARSQL_TRACE` environment variable to track executed SQL queries:

```bash
PARSQL_TRACE=1 cargo run
```

This will print all executed SQL queries to the console.

### Macro Options

The macros provide various features to offer flexibility in SQL generation:

#### Queryable (SELECT)

```rust
#[derive(Queryable, FromRow, SqlParams)]
#[table("users")]
#[select("SELECT * FROM users")] // Optional custom SQL
#[where_clause("id = $ AND state = $")] // Conditions
#[order_by("id DESC")] // Sorting
#[limit(10)] // Limit
#[offset(5)] // Offset
struct UserQuery {
    // ...
}
```

#### Insertable

```rust
#[derive(Insertable, SqlParams)]
#[table("users")]
#[returning("id")] // For INSERT...RETURNING
struct NewUser {
    // ...
}
```

#### Updateable

```rust
#[derive(Updateable, UpdateParams)]
#[table("users")]
#[update("name, email")] // Only update specific fields
#[where_clause("id = $")]
struct UpdateUser {
    // ...
}
```

## Performance Tips

* Reuse queries with the same SQL structure to take advantage of the query plan cache
* Use connection pools for large numbers of queries
* Use pagination (limit and offset) instead of `get_all` for large datasets
* Apply filters at the database level, not in your application

## Error Catching and Handling

```rust
async fn handle_database() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=test",
        NoTls,
    ).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
    
    let result = match get(&client, user_query).await {
        Ok(user) => {
            println!("User found: {:?}", user);
            // Operation successful
            Ok(())
        },
        Err(e) => match e.code() {
            // Handle specific PostgreSQL error codes
            Some(code) if code == &tokio_postgres::error::SqlState::UNIQUE_VIOLATION => {
                println!("Uniqueness violation: {}", e);
                Err(e.into())
            },
            Some(code) if code == &tokio_postgres::error::SqlState::FOREIGN_KEY_VIOLATION => {
                println!("Foreign key violation: {}", e);
                Err(e.into())
            },
            _ => {
                println!("General database error: {}", e);
                Err(e.into())
            }
        },
    };
    
    result
}
```

## Licensing

This library is licensed under the MIT or Apache-2.0 license. 