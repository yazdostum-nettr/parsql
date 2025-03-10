// use parsql_core::{Deleteable, Insertable, Queryable, Updateable};
use deadpool_postgres::Transaction;
use tokio_postgres::Error;
// Makrolar sadece dokümantasyon için kullanılıyor, gerçek kodda SqlQuery kullanılmalı
// use parsql_macros::{Insertable, Updateable};

use crate::{SqlQuery, SqlParams};

/// # tx_update
/// 
/// Updates a record within a transaction.
/// 
/// ## Parameters
/// - `transaction`: Active transaction object
/// - `entity`: Data object containing the update information (must implement Updateable and SqlParams traits)
/// 
/// ## Return Value
/// - `Result<(Transaction<'_>, u64), Error>`: On success, returns the transaction and number of updated records
/// 
/// ## Example Usage
/// ```rust,no_run
/// use tokio_postgres::{NoTls, Error};
/// use deadpool_postgres::{Config, Runtime};
/// use parsql::tokio_postgres::tx_update;
/// 
/// #[derive(Updateable, UpdateParams)]
/// #[table("users")]
/// #[update("name, email")]
/// #[where_clause("id = $")]
/// pub struct UpdateUser {
///     pub id: i32,
///     pub name: String,
///     pub email: String,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let mut cfg = Config::new();
///     cfg.host = Some("localhost".to_string());
///     cfg.dbname = Some("test".to_string());
///     
///     let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
///     let client = pool.get().await?;
///     
///     let mut tx = client.transaction().await?;
///     
///     let update_user = UpdateUser {
///         id: 1,
///         name: String::from("John"),
///         email: String::from("john@example.com"),
///     };
///     
///     let (tx, rows_affected) = tx_update(tx, update_user).await?;
///     tx.commit().await?;
///     
///     println!("Updated {} rows", rows_affected);
///     Ok(())
/// }
/// ```
pub async fn tx_update<T: SqlQuery + SqlParams>(
    transaction: Transaction<'_>,
    entity: T,
) -> Result<(Transaction<'_>, u64), Error> {
    let sql = T::query();
    
    if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
        println!("[PARSQL-TOKIO-POSTGRES-TX] Execute SQL: {}", sql);
    }

    let params = entity.params();
    let result = transaction.execute(&sql, &params).await?;
    Ok((transaction, result))
}

/// # tx_insert
/// 
/// Inserts a record within a transaction.
/// 
/// ## Parameters
/// - `transaction`: Active transaction object
/// - `entity`: Data object to be inserted (must implement Insertable and SqlParams traits)
/// 
/// ## Return Value
/// - `Result<(Transaction<'_>, u64), Error>`: On success, returns the transaction and number of inserted records
/// 
/// ## Example Usage
/// ```rust,no_run
/// use tokio_postgres::{NoTls, Error};
/// use deadpool_postgres::{Config, Runtime};
/// use parsql::tokio_postgres::tx_insert;
/// 
/// #[derive(Insertable, SqlParams)]
/// #[table("users")]
/// pub struct InsertUser {
///     pub name: String,
///     pub email: String,
///     pub state: i16,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let mut cfg = Config::new();
///     cfg.host = Some("localhost".to_string());
///     cfg.dbname = Some("test".to_string());
///     
///     let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
///     let client = pool.get().await?;
///     
///     let mut tx = client.transaction().await?;
///     
///     let insert_user = InsertUser {
///         name: "John".to_string(),
///         email: "john@example.com".to_string(),
///         state: 1,
///     };
///     
///     let (tx, rows_affected) = tx_insert(tx, insert_user).await?;
///     tx.commit().await?;
///     
///     println!("Inserted {} rows", rows_affected);
///     Ok(())
/// }
/// ```
pub async fn tx_insert<T: SqlQuery + SqlParams>(
    transaction: Transaction<'_>,
    entity: T,
) -> Result<(Transaction<'_>, u64), Error> {
    let sql = T::query();
    
    if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
        println!("[PARSQL-TOKIO-POSTGRES-TX] Execute SQL: {}", sql);
    }

    let params = entity.params();
    let result = transaction.execute(&sql, &params).await?;
    Ok((transaction, result))
}