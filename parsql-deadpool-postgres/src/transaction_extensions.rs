use std::fmt::Debug;
use tokio_postgres::Error;
use deadpool_postgres::Transaction;

use crate::{SqlQuery, SqlParams, FromRow, UpdateParams};

/// TransactionOps trait, Transaction için CRUD işlemlerini extension method olarak sağlar
/// Bu şekilde, herhangi bir Transaction nesnesi üzerinde doğrudan CRUD işlemleri yapılabilir
pub trait TransactionOps {
    /// Insert method, yeni bir kayıt eklemek için kullanılır
    ///
    /// # Parameters
    /// * `entity` - Eklenecek varlık, SqlQuery ve SqlParams trait'lerini implement etmeli
    ///
    /// # Örnek Kullanım
    /// ```rust,no_run
    /// use deadpool_postgres::{Config, Runtime};
    /// use parsql_deadpool_postgres::TransactionOps;
    /// use tokio_postgres::NoTls;
    ///
    /// // Entity tanımı
    /// #[derive(SqlQuery, SqlParams)]
    /// #[table("users")]
    /// pub struct InsertUser {
    ///     pub name: String,
    ///     pub email: String,
    /// }
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), tokio_postgres::Error> {
    ///     let mut cfg = Config::new();
    ///     cfg.dbname = Some("test".to_string());
    ///     let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    ///     
    ///     let client = pool.get().await?;
    ///     let tx = client.transaction().await?;
    ///     
    ///     let user = InsertUser {
    ///         name: "John".to_string(),
    ///         email: "john@example.com".to_string(),
    ///     };
    ///     
    ///     // Extension method kullanımı
    ///     let rows_affected = tx.insert(user).await?;
    ///     tx.commit().await?;
    ///     
    ///     println!("{} kayıt eklendi", rows_affected);
    ///     Ok(())
    /// }
    /// ```
    async fn insert<T>(&self, entity: T) -> Result<u64, Error>
    where
        T: SqlQuery + SqlParams + Debug;

    /// Update method, mevcut bir kaydı güncellemek için kullanılır
    ///
    /// # Parameters
    /// * `entity` - Güncellenecek varlık, SqlQuery, UpdateParams ve SqlParams trait'lerini implement etmeli
    async fn update<T>(&self, entity: T) -> Result<u64, Error>
    where
        T: SqlQuery + UpdateParams + SqlParams + Debug;

    /// Delete method, bir kaydı silmek için kullanılır
    ///
    /// # Parameters
    /// * `entity` - Silinecek varlık, SqlQuery ve SqlParams trait'lerini implement etmeli
    async fn delete<T>(&self, entity: T) -> Result<u64, Error>
    where
        T: SqlQuery + SqlParams + Debug;

    /// Get method, tek bir kayıt getirmek için kullanılır
    ///
    /// # Parameters
    /// * `params` - Sorgu parametreleri, SqlQuery, FromRow ve SqlParams trait'lerini implement etmeli
    async fn get<T>(&self, params: &T) -> Result<T, Error>
    where
        T: SqlQuery + FromRow + SqlParams + Debug;

    /// Get All method, birden fazla kayıt getirmek için kullanılır
    ///
    /// # Parameters
    /// * `params` - Sorgu parametreleri, SqlQuery, FromRow ve SqlParams trait'lerini implement etmeli
    async fn get_all<T>(&self, params: &T) -> Result<Vec<T>, Error>
    where
        T: SqlQuery + FromRow + SqlParams + Debug;

    /// Select method, özel dönüşüm fonksiyonu ile tek bir kayıt getirmek için kullanılır
    ///
    /// # Parameters
    /// * `entity` - Sorgu parametreleri, SqlQuery ve SqlParams trait'lerini implement etmeli
    /// * `to_model` - Satırı istenen türe dönüştüren fonksiyon
    async fn select<T, R, F>(&self, entity: T, to_model: F) -> Result<R, Error>
    where
        T: SqlQuery + SqlParams + Debug,
        F: FnOnce(&tokio_postgres::Row) -> Result<R, Error>;

    /// Select All method, özel dönüşüm fonksiyonu ile birden fazla kayıt getirmek için kullanılır
    ///
    /// # Parameters
    /// * `entity` - Sorgu parametreleri, SqlQuery ve SqlParams trait'lerini implement etmeli
    /// * `to_model` - Her satırı istenen türe dönüştüren fonksiyon
    async fn select_all<T, R, F>(&self, entity: T, to_model: F) -> Result<Vec<R>, Error>
    where
        T: SqlQuery + SqlParams + Debug,
        F: Fn(&tokio_postgres::Row) -> R;
}

impl<'a> TransactionOps for Transaction<'a> {
    async fn insert<T>(&self, entity: T) -> Result<u64, Error>
    where
        T: SqlQuery + SqlParams + Debug,
    {
        let sql = T::query();
        
        if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
            println!("[PARSQL-DEADPOOL-POSTGRES-TX] Execute SQL: {} {:?}", sql, entity);
        }

        let params = SqlParams::params(&entity);
        self.execute(&sql, &params[..]).await
    }

    async fn update<T>(&self, entity: T) -> Result<u64, Error>
    where
        T: SqlQuery + UpdateParams + SqlParams + Debug,
    {
        let sql = T::query();
        
        if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
            println!("[PARSQL-DEADPOOL-POSTGRES-TX] Execute SQL: {} {:?}", sql, entity);
        }

        let params = SqlParams::params(&entity);
        self.execute(&sql, &params[..]).await
    }

    async fn delete<T>(&self, entity: T) -> Result<u64, Error>
    where
        T: SqlQuery + SqlParams + Debug,
    {
        let sql = T::query();
        
        if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
            println!("[PARSQL-DEADPOOL-POSTGRES-TX] Execute SQL: {} {:?}", sql, entity);
        }

        let params = SqlParams::params(&entity);
        self.execute(&sql, &params[..]).await
    }

    async fn get<T>(&self, params: &T) -> Result<T, Error>
    where
        T: SqlQuery + FromRow + SqlParams + Debug,
    {
        let sql = T::query();
        
        if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
            println!("[PARSQL-DEADPOOL-POSTGRES-TX] Execute SQL: {} {:?}", sql, params);
        }

        let query_params = SqlParams::params(params);
        let row = self.query_one(&sql, &query_params[..]).await?;
        T::from_row(&row)
    }

    async fn get_all<T>(&self, params: &T) -> Result<Vec<T>, Error>
    where
        T: SqlQuery + FromRow + SqlParams + Debug,
    {
        let sql = T::query();
        
        if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
            println!("[PARSQL-DEADPOOL-POSTGRES-TX] Execute SQL: {} {:?}", sql, params);
        }

        let query_params = SqlParams::params(params);
        let rows = self.query(&sql, &query_params[..]).await?;
        
        let mut results = Vec::with_capacity(rows.len());
        for row in rows {
            results.push(T::from_row(&row)?);
        }
        
        Ok(results)
    }

    async fn select<T, R, F>(&self, entity: T, to_model: F) -> Result<R, Error>
    where
        T: SqlQuery + SqlParams + Debug,
        F: FnOnce(&tokio_postgres::Row) -> Result<R, Error>,
    {
        let sql = T::query();
        
        if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
            println!("[PARSQL-DEADPOOL-POSTGRES-TX] Execute SQL: {} {:?}", sql, entity);
        }

        let params = SqlParams::params(&entity);
        let row = self.query_one(&sql, &params[..]).await?;
        to_model(&row)
    }

    async fn select_all<T, R, F>(&self, entity: T, to_model: F) -> Result<Vec<R>, Error>
    where
        T: SqlQuery + SqlParams + Debug,
        F: Fn(&tokio_postgres::Row) -> R,
    {
        let sql = T::query();
        
        if std::env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
            println!("[PARSQL-DEADPOOL-POSTGRES-TX] Execute SQL: {} {:?}", sql, entity);
        }

        let params = SqlParams::params(&entity);
        let rows = self.query(&sql, &params[..]).await?;
        
        let mut results = Vec::with_capacity(rows.len());
        for row in rows {
            results.push(to_model(&row));
        }
        
        Ok(results)
    }
} 