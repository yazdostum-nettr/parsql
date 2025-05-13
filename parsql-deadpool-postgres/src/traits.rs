use postgres::types::FromSql;
use tokio_postgres::{Error, Row};
use tokio_postgres::types::ToSql;
use std::fmt::Debug;
use async_trait::async_trait;

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
    fn from_row(row: &Row) -> Result<Self, Error>
    where
        Self: Sized;
}

/// CrudOps trait'i, Pool nesnesi için CRUD işlemlerini extension method olarak sağlar.
/// Bu trait, Pool üzerinde doğrudan CRUD işlemlerini çağırmayı mümkün kılar.
#[async_trait]
pub trait CrudOps {
    /// Veritabanına yeni bir kayıt ekler.
    async fn insert<T, P:for<'a> FromSql<'a> + Send + Sync>(&self, entity: T) -> Result<P, Error>
    where
        T: SqlQuery + SqlParams + Send + Sync;
    
    /// Veritabanındaki mevcut bir kaydı günceller.
    async fn update<T>(&self, entity: T) -> Result<u64, Error>
    where
        T: SqlQuery + UpdateParams + Send + Sync;
    
    /// Veritabanından bir kaydı siler.
    async fn delete<T>(&self, entity: T) -> Result<u64, Error>
    where
        T: SqlQuery + SqlParams + Send + Sync;
    
    /// Belirtilen kriterlere uygun tek bir kaydı getirir.
    async fn fetch<T>(&self, params: &T) -> Result<T, Error>
    where
        T: SqlQuery + FromRow + SqlParams + Send + Sync;
    
    /// Belirtilen kriterlere uygun tüm kayıtları getirir.
    async fn fetch_all<T>(&self, params: &T) -> Result<Vec<T>, Error>
    where
        T: SqlQuery + FromRow + SqlParams + Send + Sync;
    
    /// Belirtilen özel dönüşüm fonksiyonunu kullanarak tek bir kaydı getirir.
    async fn select<T, R, F>(&self, entity: T, to_model: F) -> Result<R, Error>
    where
        T: SqlQuery + SqlParams + Send + Sync,
        F: FnOnce(&Row) -> Result<R, Error> + Send + Sync;
    
    /// Belirtilen özel dönüşüm fonksiyonunu kullanarak tüm kayıtları getirir.
    async fn select_all<T, R, F>(&self, entity: T, to_model: F) -> Result<Vec<R>, Error>
    where
        T: SqlQuery + SqlParams + Send + Sync,
        F: Fn(&Row) -> R + Send + Sync;
}

/// TransactionOps trait, Transaction için CRUD işlemlerini extension method olarak sağlar
/// Bu şekilde, herhangi bir Transaction nesnesi üzerinde doğrudan CRUD işlemleri yapılabilir
#[async_trait]
pub trait TransactionOps {
    /// Insert method, yeni bir kayıt eklemek için kullanılır
    async fn insert<T>(&self, entity: T) -> Result<u64, Error>
    where
        T: SqlQuery + SqlParams + Debug + Send + 'static;

    /// Update method, mevcut bir kaydı güncellemek için kullanılır
    async fn update<T>(&self, entity: T) -> Result<u64, Error>
    where
        T: SqlQuery + UpdateParams + SqlParams + Debug + Send + 'static;

    /// Delete method, bir kaydı silmek için kullanılır
    async fn delete<T>(&self, entity: T) -> Result<u64, Error>
    where
        T: SqlQuery + SqlParams + Debug + Send + 'static;

    /// Get method, tek bir kayıt getirmek için kullanılır
    async fn get<T>(&self, params: &T) -> Result<T, Error>
    where
        T: SqlQuery + FromRow + SqlParams + Debug + Send + Sync + Clone + 'static;

    /// Get All method, birden fazla kayıt getirmek için kullanılır
    async fn get_all<T>(&self, params: &T) -> Result<Vec<T>, Error>
    where
        T: SqlQuery + FromRow + SqlParams + Debug + Send + Sync + Clone + 'static;

    /// Select method, özel dönüşüm fonksiyonu ile tek bir kayıt getirmek için kullanılır
    async fn select<T, R, F>(&self, entity: T, to_model: F) -> Result<R, Error>
    where
        T: SqlQuery + SqlParams + Debug + Send + 'static,
        F: FnOnce(&Row) -> Result<R, Error> + Send + Sync + 'static,
        R: Send + 'static;

    /// Select All method, özel dönüşüm fonksiyonu ile birden fazla kayıt getirmek için kullanılır
    async fn select_all<T, R, F>(&self, entity: T, to_model: F) -> Result<Vec<R>, Error>
    where
        T: SqlQuery + SqlParams + Debug + Send + 'static,
        F: Fn(&Row) -> R + Send + Sync + 'static,
        R: Send + 'static;
}
