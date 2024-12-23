use parsql_core::{Deleteable, Insertable, Queryable, Updateable};
use rusqlite::{Error, Row, ToSql};

pub trait SqlParams {
    fn params(&self) -> Vec<&(dyn ToSql + Sync)>;
}

pub fn insert<T: Insertable + SqlParams>(
    conn: &rusqlite::Connection,
    entity: T,
) -> Result<usize, rusqlite::Error> {
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

    let _params: Vec<&dyn ToSql> = entity.params().iter().map(|p| *p as &dyn ToSql).collect();

    match conn.execute(&sql, _params.as_slice()) {
        Ok(_result) => Ok(_result),
        Err(_err) => panic!("Insert işlemi yürütme esnasında bir hata oluştu! {}", _err),
    }
}

pub fn update<T: Updateable + SqlParams>(
    conn: &rusqlite::Connection,
    entity: T,
) -> Result<usize, Error> {
    let table_name = T::table_name();
    let columns = T::update_clause();
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

    let _params: Vec<&dyn ToSql> = entity.params().iter().map(|p| *p as &dyn ToSql).collect();

    match conn.execute(&sql, _params.as_slice()) {
        Ok(rows_affected) => Ok(rows_affected),
        Err(e) => Err(e),
    }
}

pub fn delete<T: Deleteable + SqlParams>(
    conn: &rusqlite::Connection,
    entity: T,
) -> Result<usize, Error> {
    let table_name = T::table_name();
    let where_clause = T::where_clause();

    let sql = format!("DELETE FROM {} WHERE {}", table_name, where_clause);

    let _params: Vec<&dyn ToSql> = entity.params().iter().map(|p| *p as &dyn ToSql).collect();

    match conn.execute(&sql, _params.as_slice()) {
        Ok(rows_affected) => Ok(rows_affected),
        Err(e) => Err(e),
    }
}

pub fn get<T: Queryable + SqlParams, F, R>(
    conn: &rusqlite::Connection,
    entity: T,
    to_model: F,
) -> Result<R, Error>
where
    F: Fn(&Row) -> Result<R, Error>,
{
    let table_name = T::table_name();
    let select_clause = T::select_clause().join(", ");
    let where_clause = T::where_clause();

    let sql = format!(
        "SELECT {} FROM {} WHERE {}",
        select_clause, table_name, where_clause
    );

    let _params: Vec<&dyn ToSql> = entity.params().iter().map(|p| *p as &dyn ToSql).collect();

    match conn.query_row(&sql, _params.as_slice(), |row| to_model(row)) {
        Ok(row) => Ok(row),
        Err(e) => Err(e),
    }
}

pub fn get_all<T: Queryable + SqlParams, F, R>(
    conn: &rusqlite::Connection,
    entity: T,
    to_model: F,
) -> Result<Vec<R>, Error>
where
    F: Fn(&Row) -> Result<R, Error>,
{
    let table = T::table_name();
    let columns = T::select_clause().join(", ");
    let where_clause = T::where_clause();

    let sql = format!("SELECT {} FROM {} WHERE {}", columns, table, where_clause);

    let _params: Vec<&dyn ToSql> = entity.params().iter().map(|p| *p as &dyn ToSql).collect();

    let mut stmt = conn.prepare(&sql).unwrap();

    stmt.query_map([], |row| to_model(row))
        .map(|iter| iter.collect::<Result<Vec<R>, _>>())
        .map_err(|err| println!("{:?}", err))
        .unwrap()
}
