//! # parsql-postgres
//!
//! Synchronous PostgreSQL integration for parsql.
//! This crate provides synchronous APIs for working with PostgreSQL databases.
//!
//! ## Features
//!
//! - Synchronous PostgreSQL operations
//! - Automatic SQL query generation
//! - Secure parameter management
//! - Generic CRUD operations
//! - Transaction support
//! - Extension methods for the Client object
//!
//! ## Usage
//!
//! ```rust,no_run
//! use postgres::{Client, NoTls, Error};
//! use parsql::postgres::{get, insert};
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
//! #[where_clause("id = $")]
//! pub struct GetUser {
//!     pub id: i32,
//!     pub name: String,
//!     pub email: String,
//! }
//!
//! fn main() -> Result<(), Error> {
//!     let mut client = Client::connect(
//!         "host=localhost user=postgres dbname=test",
//!         NoTls,
//!     )?;
//!     
//!     // Insert a new user
//!     let insert_user = InsertUser {
//!         name: "John".to_string(),
//!         email: "john@example.com".to_string(),
//!     };
//!     
//!     let id = insert(&mut client, insert_user)?;
//!     
//!     // Get the user back
//!     let get_user = GetUser::new(id as i32);
//!     let user = get(&mut client, &get_user)?;
//!     
//!     println!("User: {:?}", user);
//!     Ok(())
//! }
//! ```
//!
//! ## Using Extension Methods
//!
//! You can also use the extension methods directly on the Client object:
//!
//! ```rust,no_run
//! use postgres::{Client, NoTls, Error};
//! use parsql::postgres::CrudOps;  // Import the trait
//! use parsql::macros::{Insertable, SqlParams, Queryable, FromRow};
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
//! #[where_clause("id = $1")]
//! pub struct GetUser {
//!     pub id: i32,
//!     pub name: String,
//!     pub email: String,
//! }
//!
//! fn main() -> Result<(), Error> {
//!     let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
//!     
//!     // Insert a new user using extension method
//!     let insert_user = InsertUser {
//!         name: "John".to_string(),
//!         email: "john@example.com".to_string(),
//!     };
//!     
//!     let rows_affected = client.insert(insert_user)?;
//!     
//!     // Get the user back using extension method
//!     let get_user = GetUser {
//!         id: 1,
//!         name: String::new(),
//!         email: String::new(),
//!     };
//!     let user = client.get(&get_user)?;
//!     
//!     println!("User: {:?}", user);
//!     Ok(())
//! }
//! ```
//!
//! ## Using Transactions
//!
//! This crate supports transaction operations in two ways: through the `CrudOps` trait
//! methods directly on a `Transaction` object, or through the helper functions provided
//! in the `transactional` module.
//!
//! ### Using CrudOps with Transaction
//!
//! ```rust,no_run
//! use postgres::{Client, NoTls, Error};
//! use parsql::postgres::CrudOps;
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
//! #[where_clause("id = $")]
//! struct UpdateUser {
//!     id: i32,
//!     email: String,
//! }
//!
//! fn main() -> Result<(), Error> {
//!     let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
//!     
//!     // Start a transaction
//!     let mut tx = client.transaction()?;
//!     
//!     // Use CrudOps methods directly on the transaction
//!     let insert_user = InsertUser {
//!         name: "John".to_string(),
//!         email: "john@example.com".to_string(),
//!     };
//!     let rows_affected = tx.insert(insert_user)?;
//!     
//!     let update_user = UpdateUser {
//!         id: 1,
//!         email: "john.updated@example.com".to_string(),
//!     };
//!     let rows_updated = tx.update(update_user)?;
//!     
//!     // Commit the transaction
//!     tx.commit()?;
//!     Ok(())
//! }
//! ```
//!
//! ### Using Transaction Helper Functions
//!
//! ```rust,no_run
//! use postgres::{Client, NoTls, Error};
//! use parsql::postgres::transactional::{begin, tx_insert, tx_update};
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
//! #[where_clause("id = $")]
//! struct UpdateUser {
//!     id: i32,
//!     email: String,
//! }
//!
//! fn main() -> Result<(), Error> {
//!     let mut client = Client::connect("host=localhost user=postgres", NoTls)?;
//!     
//!     // Begin a transaction
//!     let tx = begin(&mut client)?;
//!     
//!     // Chain transaction operations
//!     let insert_user = InsertUser {
//!         name: "John".to_string(),
//!         email: "john@example.com".to_string(),
//!     };
//!     
//!     let (tx, _) = tx_insert(tx, insert_user)?;
//!     
//!     let update_user = UpdateUser {
//!         id: 1,
//!         email: "john.updated@example.com".to_string(),
//!     };
//!     
//!     let (tx, _) = tx_update(tx, update_user)?;
//!     
//!     // Commit the transaction
//!     tx.commit()?;
//!     Ok(())
//! }
//! ```

pub mod crud_ops;
pub mod transaction_ops;

pub use postgres::types::ToSql;
pub use postgres::Transaction;
pub use postgres::{Client, Error, Row};

// Re-export crud operations
pub use crud_ops::{
    delete, get, get_all, get_by_query, insert, select, select_all, update, CrudOps,
};

// Re-export transaction operations in a transactional module
pub mod transactional {
    pub use crate::transaction_ops::{
        begin, tx_delete, tx_get, tx_get_all, tx_insert, tx_select, tx_select_all, tx_update,
    };
}

pub use parsql_macros as macros;

/// Trait for generating SQL queries.
/// This trait is implemented by the derive macro `Queryable`, `Insertable`, `Updateable`, and `Deletable`.
pub trait SqlQuery {
    /// Returns the SQL query string.
    fn query() -> String;
}

/// Trait for providing SQL parameters.
/// This trait is implemented by the derive macro `SqlParams`.
pub trait SqlParams {
    /// Returns a vector of references to SQL parameters.
    fn params(&self) -> Vec<&(dyn postgres::types::ToSql + Sync)>;
}

/// Trait for providing UPDATE parameters.
/// This trait is implemented by the derive macro `UpdateParams`.
pub trait UpdateParams {
    /// Returns a vector of references to SQL parameters for UPDATE operations.
    fn params(&self) -> Vec<&(dyn postgres::types::ToSql + Sync)>;
}

/// Trait for converting database rows to Rust structs.
/// This trait is implemented by the derive macro `FromRow`.
pub trait FromRow {
    /// Converts a database row to a Rust struct.
    ///
    /// # Arguments
    /// * `row` - A reference to a database row
    ///
    /// # Returns
    /// * `Result<Self, Error>` - The converted struct or an error
    fn from_row(row: &postgres::Row) -> Result<Self, postgres::Error>
    where
        Self: Sized;
}
