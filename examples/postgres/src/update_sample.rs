use parsql::postgres::{
    macros::{UpdateParams, Updateable},
    traits::{SqlQuery, UpdateParams},
};
use postgres::types::ToSql;

#[derive(Updateable, UpdateParams)]
#[table("users")]
#[update("name, email")]
#[where_clause("id = $")]
pub struct UpdateUser {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub state: i16,
}
