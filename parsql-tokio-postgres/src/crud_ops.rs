use tokio_postgres::{Error, Row, Client, Transaction};
use crate::{SqlQuery, SqlParams, UpdateParams, FromRow};

/// A trait for extending PostgreSQL client with CRUD operations.
///
/// This trait provides extension methods for tokio_postgres::Client to perform
/// common database CRUD operations in a more ergonomic way.
pub trait CrudOps {
    /// Inserts a new record into the database.
    ///
    /// # Arguments
    /// * `entity` - Data object to be inserted (must implement SqlQuery and SqlParams traits)
    ///
    /// # Return Value
    /// * `Result<u64, Error>` - On success, returns the number of inserted records; on failure, returns Error
    ///
    /// # Example
    /// ```rust,no_run
    /// # use tokio_postgres::{NoTls, Client};
    /// # use parsql::tokio_postgres::CrudOps;
    /// # use parsql::macros::{Insertable, SqlParams};
    /// #
    /// #[derive(Insertable, SqlParams)]
    /// #[table("users")]
    /// struct InsertUser {
    ///     name: String,
    ///     email: String,
    /// }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let (client, connection) = tokio_postgres::connect("", NoTls).await?;
    /// # tokio::spawn(async move { connection.await; });
    /// let user = InsertUser {
    ///     name: "John".to_string(),
    ///     email: "john@example.com".to_string(),
    /// };
    ///
    /// let id = client.insert(user).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn insert<T: SqlQuery + SqlParams>(&self, entity: T) -> Result<u64, Error>;

    /// Updates an existing record in the database.
    ///
    /// # Arguments
    /// * `entity` - Data object containing the update information (must implement SqlQuery and UpdateParams traits)
    ///
    /// # Return Value
    /// * `Result<bool, Error>` - On success, returns true if at least one record was updated; on failure, returns Error
    ///
    /// # Example
    /// ```rust,no_run
    /// # use tokio_postgres::{NoTls, Client};
    /// # use parsql::tokio_postgres::CrudOps;
    /// # use parsql::macros::{Updateable, UpdateParams};
    /// #
    /// #[derive(Updateable, UpdateParams)]
    /// #[table("users")]
    /// #[update("name, email")]
    /// #[where_clause("id = $")]
    /// struct UpdateUser {
    ///     id: i64,
    ///     name: String,
    ///     email: String,
    /// }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let (client, connection) = tokio_postgres::connect("", NoTls).await?;
    /// # tokio::spawn(async move { connection.await; });
    /// let user = UpdateUser {
    ///     id: 1,
    ///     name: "John Smith".to_string(),
    ///     email: "john.smith@example.com".to_string(),
    /// };
    ///
    /// let updated = client.update(user).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn update<T: SqlQuery + UpdateParams>(&self, entity: T) -> Result<bool, Error>;

    /// Deletes a record from the database.
    ///
    /// # Arguments
    /// * `entity` - Data object containing delete conditions (must implement SqlQuery and SqlParams traits)
    ///
    /// # Return Value
    /// * `Result<u64, Error>` - On success, returns the number of deleted records; on failure, returns Error
    ///
    /// # Example
    /// ```rust,no_run
    /// # use tokio_postgres::{NoTls, Client};
    /// # use parsql::tokio_postgres::CrudOps;
    /// # use parsql::macros::{Deletable, SqlParams};
    /// #
    /// #[derive(Deletable, SqlParams)]
    /// #[table("users")]
    /// #[where_clause("id = $")]
    /// struct DeleteUser {
    ///     id: i64,
    /// }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let (client, connection) = tokio_postgres::connect("", NoTls).await?;
    /// # tokio::spawn(async move { connection.await; });
    /// let user = DeleteUser { id: 1 };
    ///
    /// let deleted = client.delete(user).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn delete<T: SqlQuery + SqlParams>(&self, entity: T) -> Result<u64, Error>;

    /// Retrieves a single record from the database and converts it to a struct.
    ///
    /// # Arguments
    /// * `params` - Data object containing query parameters (must implement SqlQuery, FromRow, and SqlParams traits)
    ///
    /// # Return Value
    /// * `Result<T, Error>` - On success, returns the retrieved record as a struct; on failure, returns Error
    ///
    /// # Example
    /// ```rust,no_run
    /// # use tokio_postgres::{NoTls, Client};
    /// # use parsql::tokio_postgres::CrudOps;
    /// # use parsql::macros::{Queryable, FromRow, SqlParams};
    /// #
    /// #[derive(Queryable, FromRow, SqlParams, Debug)]
    /// #[table("users")]
    /// #[where_clause("id = $")]
    /// struct GetUser {
    ///     id: i64,
    ///     name: String,
    ///     email: String,
    /// }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let (client, connection) = tokio_postgres::connect("", NoTls).await?;
    /// # tokio::spawn(async move { connection.await; });
    /// let query = GetUser {
    ///     id: 1,
    ///     name: Default::default(),
    ///     email: Default::default(),
    /// };
    ///
    /// let user = client.get(query).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn get<T: SqlQuery + FromRow + SqlParams>(&self, params: T) -> Result<T, Error>;

    /// Retrieves multiple records from the database and converts them to a vec of structs.
    ///
    /// # Arguments
    /// * `params` - Data object containing query parameters (must implement SqlQuery, FromRow, and SqlParams traits)
    ///
    /// # Return Value
    /// * `Result<Vec<T>, Error>` - On success, returns a vector of retrieved records; on failure, returns Error
    ///
    /// # Example
    /// ```rust,no_run
    /// # use tokio_postgres::{NoTls, Client};
    /// # use parsql::tokio_postgres::CrudOps;
    /// # use parsql::macros::{Queryable, FromRow, SqlParams};
    /// #
    /// #[derive(Queryable, FromRow, SqlParams, Debug)]
    /// #[table("users")]
    /// #[where_clause("state = $")]
    /// struct GetActiveUsers {
    ///     id: i64,
    ///     name: String,
    ///     email: String,
    ///     state: i16,
    /// }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let (client, connection) = tokio_postgres::connect("", NoTls).await?;
    /// # tokio::spawn(async move { connection.await; });
    /// let query = GetActiveUsers {
    ///     id: 0,
    ///     name: Default::default(),
    ///     email: Default::default(),
    ///     state: 1, // active users
    /// };
    ///
    /// let users = client.get_all(query).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn get_all<T: SqlQuery + FromRow + SqlParams>(&self, params: T) -> Result<Vec<T>, Error>;

    /// Executes a custom SELECT query and converts the results using the provided function.
    ///
    /// # Arguments
    /// * `entity` - Data object containing query parameters (must implement SqlQuery and SqlParams traits)
    /// * `to_model` - Function to convert a row to the desired type
    ///
    /// # Return Value
    /// * `Result<R, Error>` - On success, returns the converted record; on failure, returns Error
    ///
    /// # Example
    /// ```rust,no_run
    /// # use tokio_postgres::{NoTls, Client, Row};
    /// # use parsql::tokio_postgres::CrudOps;
    /// # use parsql::macros::{Queryable, SqlParams};
    /// #
    /// #[derive(Queryable, SqlParams)]
    /// #[table("users")]
    /// #[select("SELECT u.*, p.role FROM users u JOIN profiles p ON u.id = p.user_id")]
    /// #[where_clause("u.state = $")]
    /// struct UserQuery {
    ///     state: i16,
    /// }
    ///
    /// struct UserWithRole {
    ///     id: i64,
    ///     name: String,
    ///     role: String,
    /// }
    ///
    /// fn convert_row(row: &Row) -> Result<UserWithRole, tokio_postgres::Error> {
    ///     Ok(UserWithRole {
    ///         id: row.try_get("id")?,
    ///         name: row.try_get("name")?,
    ///         role: row.try_get("role")?,
    ///     })
    /// }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let (client, connection) = tokio_postgres::connect("", NoTls).await?;
    /// # tokio::spawn(async move { connection.await; });
    /// let query = UserQuery { state: 1 };
    ///
    /// let user = client.select(query, convert_row).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn select<T: SqlQuery + SqlParams, F, R>(&self, entity: T, to_model: F) -> Result<R, Error>
    where
        F: Fn(&Row) -> Result<R, Error>;

    /// Executes a custom SELECT query and converts all the results using the provided function.
    ///
    /// # Arguments
    /// * `entity` - Data object containing query parameters (must implement SqlQuery and SqlParams traits)
    /// * `to_model` - Function to convert a row to the desired type
    ///
    /// # Return Value
    /// * `Result<Vec<R>, Error>` - On success, returns a vector of converted records; on failure, returns Error
    ///
    /// # Example
    /// ```rust,no_run
    /// # use tokio_postgres::{NoTls, Client, Row};
    /// # use parsql::tokio_postgres::CrudOps;
    /// # use parsql::macros::{Queryable, SqlParams};
    /// #
    /// #[derive(Queryable, SqlParams)]
    /// #[table("users")]
    /// #[select("SELECT u.*, p.role FROM users u JOIN profiles p ON u.id = p.user_id")]
    /// #[where_clause("u.state = $")]
    /// struct UserQuery {
    ///     state: i16,
    /// }
    ///
    /// struct UserWithRole {
    ///     id: i64,
    ///     name: String,
    ///     role: String,
    /// }
    ///
    /// fn convert_row(row: &Row) -> UserWithRole {
    ///     UserWithRole {
    ///         id: row.get("id"),
    ///         name: row.get("name"),
    ///         role: row.get("role"),
    ///     }
    /// }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let (client, connection) = tokio_postgres::connect("", NoTls).await?;
    /// # tokio::spawn(async move { connection.await; });
    /// let query = UserQuery { state: 1 };
    ///
    /// let users = client.select_all(query, convert_row).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn select_all<T: SqlQuery + SqlParams, F, R>(&self, entity: T, to_model: F) -> Result<Vec<R>, Error>
    where
        F: Fn(&Row) -> R;
}

impl CrudOps for Client {
    async fn insert<T: SqlQuery + SqlParams>(&self, entity: T) -> Result<u64, Error> {
        let sql = T::query();
        if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
            println!("[PARSQL-TOKIO-POSTGRES] Execute SQL: {}", sql);
        }

        let params = entity.params();
        self.execute(&sql, &params).await
    }

    async fn update<T: SqlQuery + UpdateParams>(&self, entity: T) -> Result<bool, Error> {
        let sql = T::query();
        if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
            println!("[PARSQL-TOKIO-POSTGRES] Execute SQL: {}", sql);
        }

        let params = entity.params();
        let result = self.execute(&sql, &params).await?;
        Ok(result > 0)
    }

    async fn delete<T: SqlQuery + SqlParams>(&self, entity: T) -> Result<u64, Error> {
        let sql = T::query();
        if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
            println!("[PARSQL-TOKIO-POSTGRES] Execute SQL: {}", sql);
        }

        let params = entity.params();
        self.execute(&sql, &params).await
    }

    async fn get<T: SqlQuery + FromRow + SqlParams>(&self, params: T) -> Result<T, Error> {
        let sql = T::query();
        if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
            println!("[PARSQL-TOKIO-POSTGRES] Execute SQL: {}", sql);
        }

        let query_params = params.params();
        let row = self.query_one(&sql, &query_params).await?;
        T::from_row(&row)
    }

    async fn get_all<T: SqlQuery + FromRow + SqlParams>(&self, params: T) -> Result<Vec<T>, Error> {
        let sql = T::query();
        if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
            println!("[PARSQL-TOKIO-POSTGRES] Execute SQL: {}", sql);
        }

        let query_params = params.params();
        let rows = self.query(&sql, &query_params).await?;
        
        let mut results = Vec::with_capacity(rows.len());
        for row in rows {
            results.push(T::from_row(&row)?);
        }
        
        Ok(results)
    }

    async fn select<T: SqlQuery + SqlParams, F, R>(&self, entity: T, to_model: F) -> Result<R, Error>
    where
        F: Fn(&Row) -> Result<R, Error>,
    {
        let sql = T::query();
        if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
            println!("[PARSQL-TOKIO-POSTGRES] Execute SQL: {}", sql);
        }

        let params = entity.params();
        let row = self.query_one(&sql, &params).await?;
        to_model(&row)
    }

    async fn select_all<T: SqlQuery + SqlParams, F, R>(&self, entity: T, to_model: F) -> Result<Vec<R>, Error>
    where
        F: Fn(&Row) -> R,
    {
        let sql = T::query();
        if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
            println!("[PARSQL-TOKIO-POSTGRES] Execute SQL: {}", sql);
        }

        let params = entity.params();
        let rows = self.query(&sql, &params).await?;
        
        let mut results = Vec::with_capacity(rows.len());
        for row in rows {
            results.push(to_model(&row));
        }
        
        Ok(results)
    }
}
/// # insert
/// 
/// Inserts a new record into the database.
/// 
/// ## Parameters
/// - `client`: Database connection object
/// - `entity`: Data object to be inserted (must implement SqlQuery and SqlParams traits)
/// 
/// ## Return Value
/// - `Result<u64, Error>`: On success, returns the number of inserted records; on failure, returns Error
/// 
/// ## Struct Definition
/// Structs used with this function should be annotated with the following derive macros:
/// 
/// ```rust,no_run
/// #[derive(Insertable, SqlParams)]  // Required macros
/// #[table("table_name")]            // Table name to insert into
/// pub struct MyEntity {
///     pub field1: String,
///     pub field2: i32,
///     // ...
/// }
/// ```
/// 
/// - `Insertable`: Automatically generates SQL INSERT statements
/// - `SqlParams`: Automatically generates SQL parameters
/// - `#[table("table_name")]`: Specifies the table name for the insertion
/// 
/// ## Example Usage
/// ```rust,no_run
/// use tokio_postgres::{NoTls, Error};
/// use parsql::tokio_postgres::{insert};
/// use parsql::macros::{Insertable, SqlParams};
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
///     let (client, connection) = tokio_postgres::connect(
///         "host=localhost user=postgres dbname=test",
///         NoTls,
///     ).await?;
///     
///     tokio::spawn(async move {
///         if let Err(e) = connection.await {
///             eprintln!("Connection error: {}", e);
///         }
///     });
///
///     let insert_user = InsertUser {
///         name: "John".to_string(),
///         email: "john@example.com".to_string(),
///         state: 1_i16,
///     };
///
///     let insert_result = insert(&client, insert_user).await?;
///     println!("Insert result: {:?}", insert_result);
///     Ok(())
/// }
/// ```
pub async fn insert<T: SqlQuery + SqlParams>(
    client: &Client,
    entity: T,
) -> Result<u64, Error> {
    client.insert(entity).await
}

/// # update
/// 
/// Updates an existing record in the database.
/// 
/// ## Parameters
/// - `client`: Database connection object
/// - `entity`: Data object containing the update information (must implement SqlQuery and UpdateParams traits)
/// 
/// ## Return Value
/// - `Result<bool, Error>`: On success, returns true; on failure, returns Error
/// 
/// ## Struct Definition
/// Structs used with this function should be annotated with the following derive macros:
/// 
/// ```rust,no_run
/// #[derive(Updateable, UpdateParams)]  // Required macros
/// #[table("table_name")]               // Table name to update
/// #[update("field1, field2")]          // Fields to update (optional)
/// #[where_clause("id = $")]            // Update condition
/// pub struct MyEntity {
///     pub id: i32,                    // Used in where_clause
///     pub field1: String,             // Fields to update
///     pub field2: i32,
///     // ...
/// }
/// ```
/// 
/// - `Updateable`: Automatically generates SQL UPDATE statements
/// - `UpdateParams`: Automatically generates SQL parameters for UPDATE operations
/// - `#[table("table_name")]`: Specifies the table name for the update
/// - `#[update("field1, field2")]`: Specifies which fields to update (optional)
/// - `#[where_clause("id = $")]`: Specifies the condition for the update
/// 
/// ## Example Usage
/// ```rust,no_run
/// use tokio_postgres::{NoTls, Error};
/// use parsql::tokio_postgres::{update};
/// use parsql::macros::{Updateable, UpdateParams};
/// 
/// #[derive(Updateable, UpdateParams)]
/// #[table("users")]
/// #[update("name, email")]
/// #[where_clause("id = $")]
/// pub struct UpdateUser {
///     pub id: i64,
///     pub name: String,
///     pub email: String,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let (client, connection) = tokio_postgres::connect(
///         "host=localhost user=postgres dbname=test",
///         NoTls,
///     ).await?;
///     
///     tokio::spawn(async move {
///         if let Err(e) = connection.await {
///             eprintln!("Connection error: {}", e);
///         }
///     });
///
///     let update_user = UpdateUser {
///         id: 1,
///         name: "John Smith".to_string(),
///         email: "john.smith@example.com".to_string(),
///     };
///
///     let update_result = update(&client, update_user).await?;
///     println!("Update successful: {}", update_result);
///     Ok(())
/// }
/// ```
pub async fn update<T: SqlQuery + UpdateParams>(
    client: &Client,
    entity: T,
) -> Result<bool, Error> {
    client.update(entity).await
}

/// # delete
/// 
/// Deletes a record from the database.
/// 
/// ## Parameters
/// - `client`: Database connection object
/// - `entity`: Data object containing delete conditions (must implement SqlQuery and SqlParams traits)
/// 
/// ## Return Value
/// - `Result<u64, Error>`: On success, returns the number of deleted records; on failure, returns Error
/// 
/// ## Struct Definition
/// Structs used with this function should be annotated with the following derive macros:
/// 
/// ```rust,no_run
/// #[derive(Deletable, SqlParams)]   // Required macros
/// #[table("table_name")]            // Table name to delete from
/// #[where_clause("id = $")]         // Delete condition
/// pub struct MyEntity {
///     pub id: i32,                  // Used in where_clause
///     // ...
/// }
/// ```
/// 
/// - `Deletable`: Automatically generates SQL DELETE statements
/// - `SqlParams`: Automatically generates SQL parameters
/// - `#[table("table_name")]`: Specifies the table name for the deletion
/// - `#[where_clause("id = $")]`: Specifies the condition for the deletion
/// 
/// ## Example Usage
/// ```rust,no_run
/// use tokio_postgres::{NoTls, Error};
/// use parsql::tokio_postgres::{delete};
/// use parsql::macros::{Deletable, SqlParams};
/// 
/// #[derive(Deletable, SqlParams)]
/// #[table("users")]
/// #[where_clause("id = $")]
/// pub struct DeleteUser {
///     pub id: i64,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let (client, connection) = tokio_postgres::connect(
///         "host=localhost user=postgres dbname=test",
///         NoTls,
///     ).await?;
///     
///     tokio::spawn(async move {
///         if let Err(e) = connection.await {
///             eprintln!("Connection error: {}", e);
///         }
///     });
///
///     let delete_user = DeleteUser { id: 1 };
///
///     let delete_result = delete(&client, delete_user).await?;
///     println!("Number of records deleted: {}", delete_result);
///     Ok(())
/// }
/// ```
pub async fn delete<T: SqlQuery + SqlParams>(
    client: &Client,
    entity: T,
) -> Result<u64, Error> {
    client.delete(entity).await
}

/// # get
/// 
/// Retrieves a single record from the database and converts it to a struct.
/// 
/// ## Parameters
/// - `client`: Database connection object
/// - `params`: Data object containing query parameters (must implement SqlQuery, FromRow, and SqlParams traits)
/// 
/// ## Return Value
/// - `Result<T, Error>`: On success, returns the retrieved record as a struct; on failure, returns Error
/// 
/// ## Struct Definition
/// Structs used with this function should be annotated with the following derive macros:
/// 
/// ```rust,no_run
/// #[derive(Queryable, FromRow, SqlParams)]  // Required macros
/// #[table("table_name")]                   // Table name to query
/// #[where_clause("id = $")]                // Query condition
/// pub struct MyEntity {
///     pub id: i32,                        // Used in where_clause
///     pub field1: String,                 // Fields to retrieve
///     pub field2: i32,
///     // ...
/// }
/// ```
/// 
/// - `Queryable`: Automatically generates SQL SELECT statements
/// - `FromRow`: Automatically converts database rows to structs
/// - `SqlParams`: Automatically generates SQL parameters
/// - `#[table("table_name")]`: Specifies the table name for the query
/// - `#[where_clause("id = $")]`: Specifies the condition for the query
/// 
/// ## Example Usage
/// ```rust,no_run
/// use tokio_postgres::{NoTls, Error};
/// use parsql::tokio_postgres::{get};
/// use parsql::macros::{Queryable, FromRow, SqlParams};
/// 
/// #[derive(Queryable, FromRow, SqlParams, Debug)]
/// #[table("users")]
/// #[where_clause("id = $")]
/// pub struct GetUser {
///     pub id: i64,
///     pub name: String,
///     pub email: String,
/// }
///
/// impl GetUser {
///     pub fn new(id: i64) -> Self {
///         Self {
///             id,
///             name: Default::default(),
///             email: Default::default(),
///         }
///     }
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let (client, connection) = tokio_postgres::connect(
///         "host=localhost user=postgres dbname=test",
///         NoTls,
///     ).await?;
///     
///     tokio::spawn(async move {
///         if let Err(e) = connection.await {
///             eprintln!("Connection error: {}", e);
///         }
///     });
///
///     let get_user = GetUser::new(1);
///     let user = get(&client, get_user).await?;
///     println!("User: {:?}", user);
///     Ok(())
/// }
/// ```
pub async fn get<T: SqlQuery + FromRow + SqlParams>(
    client: &Client,
    params: T,
) -> Result<T, Error> {
    client.get(params).await
}

/// # get_all
/// 
/// Retrieves multiple records from the database.
/// 
/// ## Parameters
/// - `client`: Database connection object
/// - `params`: Query parameter object (must implement SqlQuery, FromRow, and SqlParams traits)
/// 
/// ## Return Value
/// - `Result<Vec<T>, Error>`: On success, returns the list of found records; on failure, returns Error
/// 
/// ## Struct Definition
/// Structs used with this function should be annotated with the following derive macros:
/// 
/// ```rust,no_run
/// #[derive(Queryable, SqlParams, FromRow, Debug)]  // Required macros
/// #[table("table_name")]                           // Table name to query
/// #[select("field1, field2, COUNT(*) as count")]   // Custom SELECT statement (optional)
/// #[join("INNER JOIN other_table ON ...")]         // JOIN statements (optional)
/// #[where_clause("status > $")]                    // Query condition
/// #[group_by("field1, field2")]                    // GROUP BY statement (optional)
/// #[having("COUNT(*) > 0")]                        // HAVING statement (optional)
/// #[order_by("count DESC")]                        // ORDER BY statement (optional)
/// pub struct MyEntity {
///     pub status: i32,                             // Field used in the query condition
///     pub field1: String,                          // Fields to be populated from the result set
///     pub field2: i32,
///     pub count: i64,                              // Calculated value
///     // ...
/// }
/// 
/// impl MyEntity {
///     pub fn new(status: i32) -> Self {
///         Self {
///             status,
///             field1: String::default(),
///             field2: 0,
///             count: 0,
///             // ...
///         }
///     }
/// }
/// ```
/// 
/// - `Queryable`: Automatically generates SQL SELECT statements
/// - `SqlParams`: Automatically generates SQL parameters
/// - `FromRow`: Enables conversion from database row to struct object
/// - `#[table("table_name")]`: Specifies the table name for the query
/// - `#[select("...")]`: Creates a custom SELECT statement (if omitted, all fields will be selected)
/// - `#[join("...")]`: Specifies JOIN statements (can be used multiple times)
/// - `#[where_clause("...")]`: Specifies the query condition (`$` will be replaced with parameter value)
/// - `#[group_by("...")]`: Specifies the GROUP BY statement
/// - `#[having("...")]`: Specifies the HAVING statement
/// - `#[order_by("...")]`: Specifies the ORDER BY statement
/// 
/// ## Example Usage
/// ```rust,no_run
/// use tokio_postgres::{NoTls, Error};
/// use parsql::tokio_postgres::{get_all};
/// 
/// // Simple query example
/// #[derive(Queryable, SqlParams, FromRow, Debug)]
/// #[table("users")]
/// #[where_clause("email = $")]
/// pub struct GetAllUsers {
///     pub id: i32,
///     pub name: String,
///     pub email: String,
///     pub state: i16,
/// }
///
/// // Complex JOIN example
/// #[derive(Queryable, SqlParams, FromRow, Debug)]
/// #[table("users")]
/// #[select("users.id, users.name, users.email, users.state as user_state, posts.id as post_id, posts.content, posts.state as post_state, comments.content as comment")]
/// #[join("INNER JOIN posts ON users.id = posts.user_id")]
/// #[join("LEFT JOIN comments ON posts.id = comments.post_id")]
/// #[where_clause("users.id = $")]
/// pub struct SelectUserWithPosts {
///     pub id: i32,
///     pub name: String,
///     pub email: String,
///     pub user_state: i16,
///     pub post_id: i32,
///     pub content: String,
///     pub post_state: i16,
///     pub comment: Option<String>,
/// }
///
/// // GROUP BY and ORDER BY example
/// #[derive(Queryable, SqlParams, FromRow, Debug)]
/// #[table("users")]
/// #[select("users.state, COUNT(*) as user_count")]
/// #[where_clause("state > $")]
/// #[group_by("users.state")]
/// #[order_by("user_count DESC")]
/// pub struct UserStateStats {
///     pub state: i16,
///     pub user_count: i64,
/// }
///
/// // HAVING filter example
/// #[derive(Queryable, SqlParams, FromRow, Debug)]
/// #[table("users")]
/// #[select("users.state, COUNT(*) as user_count")]
/// #[where_clause("state > $")]
/// #[group_by("users.state")]
/// #[having("COUNT(*) > 1")]
/// #[order_by("user_count DESC")]
/// pub struct UserStateStatsFiltered {
///     pub state: i16,
///     pub user_count: i64,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let (client, connection) = tokio_postgres::connect(
///         "host=localhost user=postgres dbname=test",
///         NoTls,
///     ).await?;
///     
///     tokio::spawn(async move {
///         if let Err(e) = connection.await {
///             eprintln!("Connection error: {}", e);
///         }
///     });
///
///     // Example usage
///     let select_user_with_posts = SelectUserWithPosts::new(1);
///     let get_user_with_posts = get_all(&client, &select_user_with_posts).await?;
///     
///     println!("Get user with posts: {:?}", get_user_with_posts);
///     
///     // Other examples
///     let user_state_stats = get_all(&client, &UserStateStats::new(0)).await?;
///     println!("User state stats: {:?}", user_state_stats);
///     
///     let user_state_stats_filtered = get_all(&client, &UserStateStatsFiltered::new(0)).await?;
///     println!("User state stats (filtered with HAVING): {:?}", user_state_stats_filtered);
///     Ok(())
/// }
/// ```
pub async fn get_all<T: SqlQuery + FromRow + SqlParams>(
    client: &Client,
    params: T,
) -> Result<Vec<T>, Error> {
    client.get_all(params).await
}

/// # select
/// 
/// Retrieves a single record from the database using a custom transformation function.
/// This is useful when you want to use a custom transformation function instead of the FromRow trait.
/// 
/// ## Parameters
/// - `client`: Database connection object
/// - `entity`: Query parameter object (must implement SqlQuery and SqlParams traits)
/// - `to_model`: Function to convert a Row object to the target object type
/// 
/// ## Return Value
/// - `Result<T, Error>`: On success, returns the transformed object; on failure, returns Error
/// 
/// ## Struct Definition
/// Structs used with this function should be annotated with the following derive macros:
/// 
/// ```rust,no_run
/// #[derive(Queryable, SqlParams)]          // Required macros (FromRow is not needed)
/// #[table("table_name")]                   // Table name to query
/// #[where_clause("id = $")]                // Query condition
/// pub struct MyQueryEntity {
///     pub id: i32,                         // Field used in the query condition
///     // Other fields can be added if necessary for the query condition
/// }
/// 
/// // A separate struct can be used for the return value
/// pub struct MyResultEntity {
///     pub id: i32,
///     pub name: String,
///     pub count: i64,
/// }
/// ```
/// 
/// - `Queryable`: Automatically generates SQL SELECT statements
/// - `SqlParams`: Automatically generates SQL parameters
/// - `#[table("table_name")]`: Specifies the table name for the query
/// - `#[where_clause("id = $")]`: Specifies the query condition (`$` will be replaced with parameter value)
/// 
/// ## Example Usage
/// ```rust,no_run
/// use tokio_postgres::{NoTls, Error};
/// use parsql::tokio_postgres::{select};
/// 
/// #[derive(Queryable, SqlParams)]
/// #[table("users")]
/// #[where_clause("id = $")]
/// pub struct UserQuery {
///     pub id: i32,
/// }
/// 
/// impl UserQuery {
///     pub fn new(id: i32) -> Self {
///         Self { id }
///     }
/// }
/// 
/// // Different return structure
/// pub struct User {
///     pub id: i32,
///     pub name: String,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let (client, connection) = tokio_postgres::connect(
///         "host=localhost user=postgres dbname=test",
///         NoTls,
///     ).await?;
///     
///     tokio::spawn(async move {
///         if let Err(e) = connection.await {
///             eprintln!("Connection error: {}", e);
///         }
///     });
///
///     // A custom model transformation function is required
///     let user_query = UserQuery::new(1);
///     let user = select(&client, user_query, |row| {
///         let id: i32 = row.get("id");
///         let name: String = row.get("name");
///         Ok(User { id, name })
///     }).await?;
///     
///     println!("User: {:?}", user);
///     Ok(())
/// }
/// ```
pub async fn select<T: SqlQuery + SqlParams, F>(
    client: &Client,
    entity: T,
    to_model: F,
) -> Result<T, Error>
where
    F: Fn(&Row) -> Result<T, Error>,
{
    client.select(entity, to_model).await
}

/// # select_all
/// 
/// Retrieves multiple records from the database using a custom transformation function.
/// This is useful when you want to use a custom transformation function instead of the FromRow trait.
/// 
/// ## Parameters
/// - `client`: Database connection object
/// - `entity`: Query parameter object (must implement SqlQuery and SqlParams traits)
/// - `to_model`: Function to convert a Row object to the target object type
/// 
/// ## Return Value
/// - `Result<Vec<T>, Error>`: On success, returns the list of transformed objects; on failure, returns Error
/// 
/// ## Struct Definition
/// Structs used with this function should be annotated with the following derive macros:
/// 
/// ```rust,no_run
/// #[derive(Queryable, SqlParams)]          // Required macros (FromRow is not needed)
/// #[table("table_name")]                   // Table name to query
/// #[select("id, name, COUNT(*) as count")] // Custom SELECT statement (optional)
/// #[where_clause("active = $")]            // Query condition
/// pub struct MyQueryEntity {
///     pub active: bool,                    // Field used in the query condition
///     // Other fields can be added if necessary for the query condition
/// }
/// 
/// // A separate struct can be used for the return value
/// pub struct MyResultEntity {
///     pub id: i32,
///     pub name: String,
///     pub count: i64,
/// }
/// ```
/// 
/// - `Queryable`: Automatically generates SQL SELECT statements
/// - `SqlParams`: Automatically generates SQL parameters
/// - `#[table("table_name")]`: Specifies the table name for the query
/// - `#[select("...")]`: Creates a custom SELECT statement (if omitted, all fields will be selected)
/// - `#[where_clause("active = $")]`: Specifies the query condition (`$` will be replaced with parameter value)
/// 
/// ## Example Usage
/// ```rust,no_run
/// use tokio_postgres::{NoTls, Error};
/// use parsql::tokio_postgres::{select_all};
/// 
/// #[derive(Queryable, SqlParams)]
/// #[table("users")]
/// #[select("id, name, email")]
/// pub struct UsersQuery {
///     // Can be empty for a parameterless query
/// }
/// 
/// impl UsersQuery {
///     pub fn new() -> Self {
///         Self {}
///     }
/// }
/// 
/// // Different return structure
/// pub struct User {
///     pub id: i32,
///     pub name: String,
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let (client, connection) = tokio_postgres::connect(
///         "host=localhost user=postgres dbname=test",
///         NoTls,
///     ).await?;
///     
///     tokio::spawn(async move {
///         if let Err(e) = connection.await {
///             eprintln!("Connection error: {}", e);
///         }
///     });
///
///     // A custom model transformation function is required
///     let users_query = UsersQuery::new();
///     let users = select_all(&client, users_query, |row| {
///         let id: i32 = row.get("id");
///         let name: String = row.get("name");
///         User { id, name }
///     }).await?;
///     
///     println!("Users: {:?}", users);
///     Ok(())
/// }
/// ```
pub async fn select_all<T: SqlQuery + SqlParams, F>(
    client: &Client,
    entity: T,
    to_model: F,
) -> Result<Vec<T>, Error>
where
    F: Fn(&Row) -> T,
{
    client.select_all(entity, to_model).await
}
