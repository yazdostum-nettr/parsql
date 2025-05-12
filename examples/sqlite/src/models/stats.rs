use parsql::sqlite::{
    macros::{FromRow, Queryable, SqlParams},
    traits::{FromRow, SqlParams, SqlQuery},
};
use rusqlite::{types::ToSql, Error, Result, Row};

/// Gelişmiş kullanıcı-gönderi istatistikleri için struct
#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[where_clause("id = $")]
pub struct UserPostStatsAdvanced {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
    pub post_count: i64,
    pub comment_count: i64,
}

impl UserPostStatsAdvanced {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            name: String::new(),
            email: String::new(),
            state: 0,
            post_count: 0,
            comment_count: 0,
        }
    }
}
