use parsql::{
    macros::{FromRow, Insertable, Queryable, SqlParams, UpdateParams, Updateable, Deletable},
    sqlite::{FromRow, SqlParams, UpdateParams, SqlQuery},
};
use rusqlite::{Result, Error, Row, types::ToSql};

/// Kullanıcı ekleme için struct
#[derive(Insertable, SqlParams)]
#[table("users")]
pub struct InsertUser {
    pub name: String,
    pub email: String,
    pub state: i16,
}

/// Kullanıcı güncelleme için struct
#[derive(Updateable, UpdateParams)]
#[table("users")]
#[update("name, email")]
#[where_clause("id = $")]
pub struct UpdateUser {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

/// ID'ye göre kullanıcı sorgulama için struct
#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[where_clause("id = $")]
pub struct GetUser {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

/// İsme göre kullanıcı sorgulama için struct
#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[where_clause("name = $")]
pub struct GetUserByName {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

/// Kullanıcı silme için struct
#[derive(Deletable, SqlParams)]
#[table("users")]
#[where_clause("id = $")]
pub struct DeleteUser {
    pub id: i64,
} 