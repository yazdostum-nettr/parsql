use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;

#[derive(Debug, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
    pub pool_size: usize,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            user: "postgres".to_string(),
            password: "postgres".to_string(),
            dbname: "parsql_example".to_string(),
            pool_size: 10,
        }
    }
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        let host = std::env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("DB_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5432);
        let user = std::env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
        let password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
        let dbname = std::env::var("DB_NAME").unwrap_or_else(|_| "parsql_example".to_string());
        let pool_size = std::env::var("DB_POOL_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10);

        Self {
            host,
            port,
            user,
            password,
            dbname,
            pool_size,
        }
    }

    pub fn create_pool(&self) -> Pool {
        let mut cfg = Config::new();
        cfg.host = Some(self.host.clone());
        cfg.port = Some(self.port);
        cfg.user = Some(self.user.clone());
        cfg.password = Some(self.password.clone());
        cfg.dbname = Some(self.dbname.clone());
        
        // Havuz yönetici yapılandırması
        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        
        cfg.manager = Some(mgr_config);
        cfg.create_pool(Some(Runtime::Tokio1), NoTls)
            .expect("Veritabanı havuzu oluşturulamadı")
    }
} 