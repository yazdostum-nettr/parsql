//! Veritabanı bağlantı işlemlerini yöneten modül.
//!
//! Bu modül, `.env` dosyasından bağlantı bilgilerini okuyarak
//! PostgreSQL veritabanına bağlantı kurulmasını sağlar.

use std::env;
use std::sync::Arc;
use tokio_postgres::{Config, NoTls, Error, Client, Connection};
use dotenvy::dotenv;

/// Bağlantı bilgilerini temsil eden struct
#[derive(Debug, Clone)]
pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            user: "postgres".to_string(),
            password: "postgres".to_string(),
            dbname: "postgres".to_string(),
        }
    }
}

impl DbConfig {
    /// `.env` dosyasından bağlantı bilgilerini okur
    pub fn from_env() -> Self {
        // .env dosyasını yükle
        if let Err(e) = dotenv() {
            eprintln!(".env dosyası yüklenemedi: {}, varsayılan değerler kullanılacak", e);
            return Self::default();
        }

        // Bağlantı parametrelerini oku
        let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("DB_PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(5432);
        let user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
        let password = env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
        let dbname = env::var("DB_NAME").unwrap_or_else(|_| "postgres".to_string());

        Self {
            host,
            port,
            user,
            password,
            dbname,
        }
    }

    /// PostgreSQL bağlantı URL'sini oluşturur
    pub fn to_connection_string(&self) -> String {
        format!(
            "host={} port={} user={} password={} dbname={}",
            self.host, self.port, self.user, self.password, self.dbname
        )
    }
}

/// Veritabanı bağlantısı oluşturur ve yönetir
pub async fn create_connection() -> Result<(Client, impl std::future::Future<Output = Result<(), Error>>), Error> {
    // .env dosyasından bağlantı bilgilerini oku
    let config = DbConfig::from_env();
    println!("Veritabanına bağlanılıyor: {}:{}/{}", config.host, config.port, config.dbname);
    
    // SQL sorgularını loglamak için PARSQL_TRACE ortam değişkenini ayarla
    if env::var("PARSQL_TRACE").unwrap_or_default() == "1" {
        env::set_var("PARSQL_TRACE", "1");
    }

    // Bağlantıyı oluştur
    let (client, connection) = tokio_postgres::connect(
        &config.to_connection_string(),
        NoTls,
    ).await?;

    Ok((client, connection))
}

/// Örnek users tablosunu oluşturur (eğer yoksa)
pub async fn setup_database(client: &Client) -> Result<(), Error> {
    println!("Veritabanı tabloları oluşturuluyor...");
    
    // Önce tabloyu sil (her çalıştırmada temiz başlamak için)
    let drop_result = client.execute("DROP TABLE IF EXISTS users", &[]).await;
    match drop_result {
        Ok(_) => println!("Eski tablo başarıyla kaldırıldı"),
        Err(e) => eprintln!("Tablo kaldırılırken hata oluştu (bu normal olabilir): {}", e),
    }
    
    // Örnek tablo oluşturma - BIGINT tipini kullanarak i64 uyumunu sağlıyoruz
    client.execute(
        "CREATE TABLE users (
            id BIGSERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            state SMALLINT NOT NULL
        )",
        &[],
    ).await?;
    
    println!("Veritabanı tabloları başarıyla oluşturuldu.");
    
    Ok(())
}

/// Tablo boşsa örnek veriler ekler
pub async fn seed_example_data_if_empty(client: &Client) -> Result<(), Error> {
    // Tabloda veri yoksa örnek veri ekleme
    let count = client.query_one("SELECT COUNT(*) FROM users", &[]).await?;
    let count: i64 = count.get(0);
    
    if count == 0 {
        println!("Örnek veri ekleniyor...");
        
        // Raw SQL ile birkaç örnek kullanıcı ekle
        client.execute(
            "INSERT INTO users (name, email, state) VALUES 
             ('Ali Yılmaz', 'ali.yilmaz@example.com', 1),
             ('Ayşe Kaya', 'ayse.kaya@example.com', 1),
             ('Mehmet Demir', 'mehmet.demir@example.com', 0)",
            &[],
        ).await?;
        
        println!("Örnek veriler eklendi.");
    }
    
    Ok(())
} 