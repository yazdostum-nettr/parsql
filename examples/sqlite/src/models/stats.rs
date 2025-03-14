use parsql::{
    macros::{FromRow, Queryable, SqlParams},
    sqlite::{FromRow, SqlParams, SqlQuery},
};
use rusqlite::{Result, Error, Row, types::ToSql};

/// Gelişmiş kullanıcı gönderisi istatistikleri için struct
#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[select("users.state, posts.state as post_state, COUNT(posts.id) as post_count, AVG(CAST(posts.id as REAL)) as avg_post_id")]
#[join("LEFT JOIN posts ON users.id = posts.user_id")]
#[where_clause("users.state > $")]
#[group_by("users.state, posts.state")]
#[having("COUNT(posts.id) > 0 AND AVG(CAST(posts.id as REAL)) > 2")]
#[order_by("post_count DESC")]
pub struct UserPostStatsAdvanced {
    pub state: i16,
    pub post_state: Option<i16>,
    pub post_count: i64,
    pub avg_post_id: Option<f32>,
}

impl UserPostStatsAdvanced {
    pub fn new(min_state: i16) -> Self {
        Self {
            state: min_state,
            post_state: None,
            post_count: 0,
            avg_post_id: None,
        }
    }
} 