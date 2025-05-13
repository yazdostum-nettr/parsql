//! # parsql-sqlite
//! 
//! SQLite integration for parsql.
//! This crate provides synchronous APIs for working with SQLite databases.
//! 
//! ## Features
//! 
//! - Synchronous SQLite operations
//! - Automatic SQL query generation
//! - Secure parameter management
//! - Generic CRUD operations
//! - Transaction support
//! - Extension methods for the Connection object
//! 
//! ## Usage
//! 
//! ```rust,no_run
//! use rusqlite::{Connection, Result};
//! use parsql::sqlite::{fetch, insert};
//! 
//! #[derive(Insertable, SqlParams)]
//! #[table("users")]
//! pub struct InsertUser {
//!     pub name: String,
//!     pub email: String,
//! }
//! 
//! #[derive(Queryable, SqlParams, FromRow)]
//! #[table("users")]
//! #[where_clause("id = ?")]
//! pub struct GetUser {
//!     pub id: i32,
//!     pub name: String,
//!     pub email: String,
//! }
//! 
//! fn main() -> Result<()> {
//!     let conn = Connection::open("test.db")?;
//!     
//!     // Insert a new user
//!     let insert_user = InsertUser {
//!         name: "John".to_string(),
//!         email: "john@example.com".to_string(),
//!     };
//!     
//!     let id = insert(&conn, insert_user)?;
//!     
//!     // Get the user back
//!     let get_user = GetUser::new(id as i32);
//!     let user = fetch(&conn, &get_user)?;
//!     
//!     println!("User: {:?}", user);
//!     Ok(())
//! }
//! ```
//!
//! ## Using Extension Methods
//!
//! You can also use the extension methods directly on the Connection object:
//!
//! ```rust,no_run
//! use rusqlite::{Connection, Result};
//! use parsql::sqlite::CrudOps;  // Import the trait
//! use parsql::sqlite::macros::{Insertable, SqlParams, Queryable, FromRow};
//!
//! #[derive(Insertable, SqlParams)]
//! #[table("users")]
//! pub struct InsertUser {
//!     pub name: String,
//!     pub email: String,
//! }
//!
//! #[derive(Queryable, FromRow, SqlParams)]
//! #[table("users")]
//! #[where_clause("id = ?")]
//! pub struct GetUser {
//!     pub id: i32,
//!     pub name: String,
//!     pub email: String,
//! }
//!
//! fn main() -> Result<()> {
//!     let conn = Connection::open("test.db")?;
//!     
//!     // Insert a new user using extension method
//!     let insert_user = InsertUser {
//!         name: "John".to_string(),
//!         email: "john@example.com".to_string(),
//!     };
//!     
//!     let rows_affected = conn.insert(insert_user)?;
//!     
//!     // Get the user back using extension method
//!     let get_user = GetUser {
//!         id: 1,
//!         name: String::new(),
//!         email: String::new(),
//!     };
//!     let user = conn.fetch(&get_user)?;
//!     
//!     println!("User: {:?}", user);
//!     Ok(())
//! }
//! ```
//!
//! ## Using Transactions
//! 
//! You can perform database operations within a transaction to ensure atomicity:
//! 
//! ```rust,no_run
//! use rusqlite::{Connection, Result};
//! use parsql::sqlite::transactional;
//! use parsql::macros::{Insertable, SqlParams, Updateable, UpdateParams};
//! 
//! #[derive(Insertable, SqlParams)]
//! #[table("users")]
//! struct InsertUser {
//!     name: String,
//!     email: String,
//! }
//! 
//! #[derive(Updateable, UpdateParams)]
//! #[table("users")]
//! #[update("email")]
//! #[where_clause("id = ?")]
//! struct UpdateUser {
//!     id: i64,
//!     email: String,
//! }
//! 
//! fn main() -> Result<()> {
//!     let conn = Connection::open("test.db")?;
//!     
//!     // Begin a transaction
//!     let tx = transactional::begin(&conn)?;
//!     
//!     // Insert a user within the transaction
//!     let insert_user = InsertUser {
//!         name: "John".to_string(),
//!         email: "john@example.com".to_string(),
//!     };
//!     let (tx, _) = transactional::tx_insert(tx, insert_user)?;
//!     
//!     // Update the user within the same transaction
//!     let update_user = UpdateUser {
//!         id: 1,
//!         email: "john.updated@example.com".to_string(),
//!     };
//!     let (tx, _) = transactional::tx_update(tx, update_user)?;
//!     
//!     // Commit the transaction - both operations succeed or fail together
//!     tx.commit()?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Installation
//!
//! Add to your Cargo.toml file as follows:
//!
//! ```toml
//! [dependencies]
//! parsql = { version = "0.3.7", features = ["sqlite"] }
//! ```
//!
//! or if you want to use this package directly:
//!
//! ```toml
//! [dependencies]
//! parsql-sqlite = "0.3.7"
//! parsql-macros = "0.3.7"
//! ```

pub mod crud_ops;
pub mod transactional_ops;
pub mod traits;
pub mod macros;

pub use macros::*;

// Re-export sqlite types that might be needed
pub use rusqlite::{Connection, Error, Row};
pub use rusqlite::types::ToSql;

// Re-export crud operations
pub use crud_ops::{
    insert, 
    select, 
    select_all, 
    update, 
    delete, 
    fetch, 
    fetch_all,
};

// Re-export transaction operations
pub use transactional_ops as transactional;
