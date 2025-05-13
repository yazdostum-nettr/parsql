use parsql::postgres::{
    macros::{Deletable, SqlParams},
    traits::{SqlParams, SqlQuery},
};
use postgres::types::ToSql;

#[derive(Deletable, SqlParams)]
#[table("users")]
#[where_clause("id = $")]
pub struct DeleteUser {
    pub id: i32,
}
