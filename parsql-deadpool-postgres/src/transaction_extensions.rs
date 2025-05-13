use std::fmt::Debug;
use std::sync::OnceLock;
use tokio_postgres::Error;
use deadpool_postgres::Transaction;
use crate::traits::{SqlQuery, SqlParams, FromRow, UpdateParams, TransactionOps};

#[async_trait::async_trait]
impl TransactionOps for Transaction<'_> {
    async fn insert<T>(&self, entity: T) -> Result<u64, Error>
    where
        T: SqlQuery + SqlParams + Debug + Send + 'static,
    {
        let sql = T::query();
        
        if let Some(trace) = std::env::var_os("PARSQL_TRACE") {
            if trace == "1" {
                println!("[PARSQL-DEADPOOL-POSTGRES-TX] Execute SQL: {}", sql);
            }
        }

        let params = SqlParams::params(&entity);
        self.execute(&sql, &params[..]).await
    }

    async fn update<T>(&self, entity: T) -> Result<u64, Error>
    where
        T: SqlQuery + UpdateParams + SqlParams + Debug + Send + 'static,
    {
        let sql = T::query();
        
        if let Some(trace) = std::env::var_os("PARSQL_TRACE") {
            if trace == "1" {
                println!("[PARSQL-DEADPOOL-POSTGRES-TX] Execute SQL: {}", sql);
            }
        }

        let params = SqlParams::params(&entity);
        self.execute(&sql, &params[..]).await
    }

    async fn delete<T>(&self, entity: T) -> Result<u64, Error>
    where
        T: SqlQuery + SqlParams + Debug + Send + 'static,
    {
        let sql = T::query();
        
        if let Some(trace) = std::env::var_os("PARSQL_TRACE") {
            if trace == "1" {
                println!("[PARSQL-DEADPOOL-POSTGRES-TX] Execute SQL: {}", sql);
            }
        }

        let params = SqlParams::params(&entity);
        self.execute(&sql, &params[..]).await
    }

    async fn get<T>(&self, params: &T) -> Result<T, Error>
    where
        T: SqlQuery + FromRow + SqlParams + Debug + Send + Sync + Clone + 'static,
    {
        let sql = T::query();
        
        static TRACE_ENABLED: OnceLock<bool> = OnceLock::new();
        let is_trace_enabled = *TRACE_ENABLED.get_or_init(|| {
            std::env::var_os("PARSQL_TRACE").map_or(false, |v| v == "1")
        });
        
        if is_trace_enabled {
            println!("[PARSQL-DEADPOOL-POSTGRES-TX] Execute SQL: {}", sql);
        }

        let params_owned = params.clone();
        let query_params = SqlParams::params(&params_owned);
        let row = self.query_one(&sql, &query_params[..]).await?;
        
        T::from_row(&row)
    }

    async fn get_all<T>(&self, params: &T) -> Result<Vec<T>, Error>
    where
        T: SqlQuery + FromRow + SqlParams + Debug + Send + Sync + Clone + 'static,
    {
        let sql = T::query();
        
        if let Some(trace) = std::env::var_os("PARSQL_TRACE") {
            if trace == "1" {
                println!("[PARSQL-DEADPOOL-POSTGRES-TX] Execute SQL: {}", sql);
            }
        }

        let params_owned = params.clone();
        let query_params = SqlParams::params(&params_owned);
        let rows = self.query(&sql, &query_params[..]).await?;
        
        let mut results = Vec::with_capacity(rows.len());
        for row in rows {
            results.push(T::from_row(&row)?);
        }
        
        Ok(results)
    }

    async fn select<T, R, F>(&self, entity: T, to_model: F) -> Result<R, Error>
    where
        T: SqlQuery + SqlParams + Debug + Send + 'static,
        F: FnOnce(&tokio_postgres::Row) -> Result<R, Error> + Send + Sync + 'static,
        R: Send + 'static,
    {
        let sql = T::query();
        
        if let Some(trace) = std::env::var_os("PARSQL_TRACE") {
            if trace == "1" {
                println!("[PARSQL-DEADPOOL-POSTGRES-TX] Execute SQL: {}", sql);
            }
        }

        let params = SqlParams::params(&entity);
        let row = self.query_one(&sql, &params[..]).await?;
        to_model(&row)
    }

    async fn select_all<T, R, F>(&self, entity: T, to_model: F) -> Result<Vec<R>, Error>
    where
        T: SqlQuery + SqlParams + Debug + Send + 'static,
        F: Fn(&tokio_postgres::Row) -> R + Send + Sync + 'static,
        R: Send + 'static,
    {
        let sql = T::query();
        
        if let Some(trace) = std::env::var_os("PARSQL_TRACE") {
            if trace == "1" {
                println!("[PARSQL-DEADPOOL-POSTGRES-TX] Execute SQL: {}", sql);
            }
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