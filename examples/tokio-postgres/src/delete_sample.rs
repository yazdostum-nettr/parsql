use parsql::tokio_postgres::{
    macros::{Deletable, SqlParams},
    traits::{SqlParams, SqlQuery},
};
use tokio_postgres::{types::ToSql, Client};

/// # DeleteUser
///
/// A struct representing a user to be deleted from the database.
///
/// ## Attributes
///
/// - `#[table("users")]`: Specifies the database table name.
/// - `#[where_clause("id = $")]`: Defines the condition for deletion.
/// - `#[derive(Debug)]`: Enables debug formatting.
///
/// ## Fields
///
/// - `id`: The unique identifier of the user to delete.
///
/// ## Example
///
/// ```rust
/// let delete_user = DeleteUser { id: 1 };
/// let result = delete(&client, delete_user).await;
/// ```
#[derive(Deletable, Debug, SqlParams)]
#[table("users")]
#[where_clause("id = $")]
pub struct DeleteUser {
    pub id: i32,
}
