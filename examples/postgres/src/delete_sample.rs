use parsql::{
    macros::{Deletable, SqlParams},
    postgres::{SqlParams, SqlQuery},
};
use postgres::types::ToSql;

#[derive(Deletable, SqlParams)]
#[table("users")]
#[where_clause("id = $")]
pub struct DeleteUser {
    pub id: i32,
}

