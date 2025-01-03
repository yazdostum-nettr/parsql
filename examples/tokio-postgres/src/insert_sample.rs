use parsql::{core::Insertable, macros::Insertable, tokio_postgres::SqlParams};
use tokio_postgres::types::ToSql;

#[derive(Insertable)]
#[table_name("users")]
pub struct InsertUser {
    pub name: String,
    pub email: String,
    pub state: i16,
}
