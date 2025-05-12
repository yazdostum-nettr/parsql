use parsql::sqlite::traits::{CrudOps, IdGenerator};
use parsql::sqlite::uuid::UuidGenerator;
use rusqlite::types::ToSql;

#[derive(Debug)]
pub struct UuidUser {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl CrudOps for UuidUser {
    fn table_name() -> &'static str {
        "uuid_users"
    }

    fn fields() -> &'static [&'static str] {
        &["id", "name", "email"]
    }

    fn values(&self) -> Vec<&dyn ToSql> {
        vec![&self.id, &self.name, &self.email]
    }
}

impl IdGenerator for UuidUser {
    type IdType = String;

    fn generate_id() -> Self::IdType {
        UuidGenerator::generate_id()
    }
} 