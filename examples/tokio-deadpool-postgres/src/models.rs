use chrono::{DateTime, Utc};
use parsql_deadpool_postgres::macros::{Deletable, Insertable, Queryable, Updateable, FromRow as DeriveFromRow, SqlParams as DeriveSqlParams, UpdateParams as DeriveUpdateParams};
use parsql_deadpool_postgres::{SqlQuery, SqlParams, UpdateParams, FromRow};
use serde::{Deserialize, Serialize};
use tokio_postgres::{types::ToSql, Row, Error};

// Kullanıcı ekleme modeli
#[derive(Debug, Clone, Serialize, Deserialize, Insertable, DeriveSqlParams)]
#[table("users")]
pub struct UserInsert {
    pub name: String,
    pub email: String,
    pub active: bool,
    pub created_at: Option<DateTime<Utc>>,
}

// Kullanıcı güncelleme modeli
#[derive(Debug, Clone, Serialize, Deserialize, Updateable, DeriveUpdateParams)]
#[table("users")]
#[update("name, email, active, updated_at")]
#[where_clause("id = $")]
pub struct UserUpdate {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub active: bool,
    pub updated_at: Option<DateTime<Utc>>,
}

// Kullanıcı silme modeli
#[derive(Debug, Clone, Serialize, Deserialize, Deletable, DeriveSqlParams)]
#[table("users")]
#[where_clause("id = $")]
pub struct UserDelete {
    pub id: i32,
}

// ID'ye göre kullanıcı getirme modeli
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, DeriveFromRow, DeriveSqlParams)]
#[table("users")]
#[select("id, name, email, active, created_at, updated_at")]
#[where_clause("id = $")]
pub struct UserById {
    pub id: i32,
    pub name: String,
    pub email: String, 
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Aktiflik durumuna göre kullanıcıları getirme modeli
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, DeriveFromRow, DeriveSqlParams)]
#[table("users")]
#[select("id, name, email, active, created_at, updated_at")]
#[where_clause("active = $")]
pub struct UsersByActive {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Kullanıcı ekleme modeli için yardımcı metotlar
impl UserInsert {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
            active: true,
            created_at: Some(Utc::now()),
        }
    }
}

// Kullanıcı güncelleme modeli için yardımcı metotlar
impl UserUpdate {
    pub fn new(id: i32, name: &str, email: &str, active: bool) -> Self {
        Self {
            id,
            name: name.to_string(),
            email: email.to_string(),
            active,
            updated_at: Some(Utc::now()),
        }
    }
}

// Kullanıcı silme modeli için yardımcı metotlar
impl UserDelete {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

// ID'ye göre kullanıcı getirme modeli için yardımcı metotlar
impl UserById {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            name: String::new(),
            email: String::new(),
            active: false,
            created_at: Utc::now(),
            updated_at: None,
        }
    }
}

// Aktiflik durumuna göre kullanıcıları getirme modeli için yardımcı metotlar
impl UsersByActive {
    pub fn new(active: bool) -> Self {
        Self {
            id: 0,
            name: String::new(),
            email: String::new(),
            active,
            created_at: Utc::now(),
            updated_at: None,
        }
    }
} 