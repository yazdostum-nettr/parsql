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
    fn params(&self) -> Vec<&(dyn ToSql + Sync)>;
}

/// Trait for providing UPDATE parameters.
/// This trait is implemented by the derive macro `UpdateParams`.
pub trait UpdateParams {
    /// Returns a vector of references to SQL parameters for UPDATE operations.
    fn params(&self) -> Vec<&(dyn ToSql + Sync)>;
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
    fn from_row(row: &tokio_postgres::Row) -> Result<Self, tokio_postgres::Error>
    where
        Self: Sized;
}

// Transaction işlemleri için modül
pub mod transactional_ops;

// Transaction işlemleri için takma ad
pub use transactional_ops as transactional;

// CRUD işlemleri için modül
mod crud_ops;

// Pool extension işlemleri için modül
pub mod pool_extensions;
pub mod transaction_extensions;

// CRUD işlemlerini dışa aktar
pub use crud_ops::{
    insert,
    update,
    delete,
    get,
    get_all,
    select,
    select_all
};

// Pool extension'ları dışa aktar
pub use pool_extensions::CrudOps;
pub use transaction_extensions::TransactionOps;

// Deadpool-postgres türlerini dışa aktar
pub use deadpool_postgres::{Pool, Client as PoolClient, PoolError, Transaction};

// Public olarak Row ve Error türlerini dışa aktar
pub use tokio_postgres::{Error, Row};
pub use tokio_postgres::types::ToSql;