use parsql::{Deleteable, Insertable, Queryable, Updateable};
use postgres::{types::ToSql, Error, Row};

pub trait SqlParams {
    fn params(&self) -> Vec<&(dyn ToSql + Sync)>;
}

pub fn insert<T: Insertable + SqlParams>(
    mut client: postgres::Client,
    entity: T,
) -> Result<u64, Error> {
    let table = T::table_name();
    let columns = T::columns().join(", ");
    let placeholders = (1..=T::columns().len())
        .map(|i| format!("${}", i))
        .collect::<Vec<_>>()
        .join(", ");

    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table, columns, placeholders
    );

    let params = entity.params();

    match client.execute(&sql, &params) {
        Ok(_id) => Ok(_id),
        Err(e) => Err(e),
    }
}

pub fn update<T: Updateable + SqlParams>(
    mut client: postgres::Client,
    entity: T,
) -> Result<u64, Error> {
    let table_name = T::table_name();
    let columns = T::updated_columns();
    let where_clause = T::where_clause();

    // Sütunları "name = $1, age = $2" formatında birleştir
    let update_clause = columns
        .iter()
        .enumerate()
        .map(|(i, col)| format!("{} = ${}", col, i + 1))
        .collect::<Vec<_>>()
        .join(", ");

    let sql = format!(
        "UPDATE {} SET {} WHERE {}",
        table_name, update_clause, where_clause
    );

    let params = entity.params();

    match client.execute(&sql, &params) {
        Ok(rows_affected) => Ok(rows_affected),
        Err(e) => Err(e),
    }
}

pub fn delete<T: Deleteable + SqlParams>(
    mut client: postgres::Client,
    entity: T,
) -> Result<u64, Error> {
    let table_name = T::table_name();
    let where_clause = T::where_clause();

    let sql = format!("DELETE FROM {} WHERE {}", table_name, where_clause);

    let params = entity.params();

    match client.execute(&sql, &params) {
        Ok(rows_affected) => Ok(rows_affected),
        Err(e) => Err(e),
    }
}

pub fn get<T: Queryable + SqlParams, F, R>(
    mut client: postgres::Client,
    entity: T,
    to_model: F,
) -> Result<R, Error>
where
    F: Fn(&Row) -> Result<R, Error>,
{
    let table_name = T::table_name();
    let select_clause = T::select_clause();
    let where_clause = T::where_clause();

    let sql = format!(
        "SELECT {} FROM {} WHERE {}",
        select_clause, table_name, where_clause
    );

    let params = entity.params();

    let query = client.prepare(&sql).unwrap();

    match client.query_one(&query, &params) {
        Ok(row) => Ok(to_model(&row)?),
        Err(e) => Err(e),
    }
}