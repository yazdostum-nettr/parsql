use deadpool_postgres::Pool;
use parsql_deadpool_postgres::{delete, get, get_all, insert, select_all, update, Error};
use tokio_postgres::Row as PgRow;

use crate::models::{UserById, UserDelete, UserInsert, UserUpdate, UsersByActive};

// Repository yapısı - Veritabanı işlemleri için
pub struct UserRepository {
    pool: Pool,
}

impl UserRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    // Kullanıcı ekleme
    pub async fn insert_user(&self, user: UserInsert) -> Result<i32, Error> {
        // Parsql'in insert fonksiyonu, doğrudan havuzla çalışır
        let result = insert(&self.pool, user).await?;
        Ok(result as i32) // Eklenen satır sayısı
    }

    // Kullanıcı güncelleme
    pub async fn update_user(&self, user: UserUpdate) -> Result<bool, Error> {
        // Parsql'in update fonksiyonu, doğrudan havuzla çalışır
        update(&self.pool, user).await
    }

    // Kullanıcı silme
    pub async fn delete_user(&self, id: i32) -> Result<u64, Error> {
        // Parsql'in delete fonksiyonu, doğrudan havuzla çalışır
        let user_delete = UserDelete::new(id);
        delete(&self.pool, user_delete).await
    }

    // ID'ye göre kullanıcı getirme
    pub async fn get_user_by_id(&self, id: i32) -> Result<UserById, Error> {
        // Parsql'in get fonksiyonu, doğrudan havuzla çalışır
        let user_query = UserById::new(id);
        get(&self.pool, &user_query).await
    }

    // Aktiflik durumuna göre kullanıcıları getirme
    pub async fn get_users_by_active(&self, active: bool) -> Result<Vec<UsersByActive>, Error> {
        // Parsql'in get_all fonksiyonu, doğrudan havuzla çalışır
        let query = UsersByActive::new(active);
        get_all(&self.pool, &query).await
    }

    // Özel sorgu ile kullanıcıları getirme
    pub async fn get_users_with_custom_transform(&self, active: bool) -> Result<Vec<UserSummary>, Error> {
        // Parsql'in select_all fonksiyonu ile özel dönüşüm yapma
        let query = UsersByActive::new(active);
        
        // Satırdan özet modeline dönüştürme fonksiyonu
        let row_to_summary = |row: &PgRow| UserSummary {
            id: row.get("id"),
            full_name: row.get("name"),
            is_active: row.get("active"),
        };
        
        select_all(&self.pool, query, row_to_summary).await
    }

    // Transaction kullanarak birden fazla işlemi atomik olarak gerçekleştirme
    pub async fn create_user_with_transaction(&self, user: UserInsert) -> Result<i32, Error> {
        // Havuzdan client al
        let mut client = self.pool.get().await.map_err(|e| {
            // PoolError'ı hata mesajına dönüştür
            eprintln!("Pool hatası: {}", e);
            Error::__private_api_timeout()
        })?;
        
        // Transaction başlat
        let tx = client.transaction().await?;
        
        // Transaction ile insert
        let sql = "INSERT INTO users (name, email, active, created_at) VALUES ($1, $2, $3, $4) RETURNING id";
        let params: &[&(dyn tokio_postgres::types::ToSql + Sync)] = &[
            &user.name,
            &user.email,
            &user.active,
            &user.created_at,
        ];
        
        let row = tx.query_one(sql, params).await?;
        let user_id: i32 = row.get(0);
        
        // Başarılıysa commit et
        tx.commit().await?;
        
        Ok(user_id)
    }
}

// Özet kullanıcı modeli - Özel dönüşüm için
#[derive(Debug, Clone)]
pub struct UserSummary {
    pub id: i32,
    pub full_name: String,
    pub is_active: bool,
}

// UserSummary için Display trait implementasyonu
impl std::fmt::Display for UserSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID: {}, Ad: {}, Aktif: {}", self.id, self.full_name, self.is_active)
    }
} 