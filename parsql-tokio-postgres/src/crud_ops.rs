use parsql_core::{Deleteable, Insertable, Queryable, Updateable};
use tokio_postgres::{Error, Row};

use crate::{SqlParams, UpdateParams, FromRow};

pub async fn insert<T: Insertable + SqlParams>(
    client: &tokio_postgres::Client,
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

    client.execute(&sql, &params).await
}

pub async fn update<T: Updateable + UpdateParams>(
    client: &tokio_postgres::Client,
    entity: T,
) -> Result<bool, Error> {
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

    let params = entity.params();

    match client.execute(&sql, &params).await {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}

pub async fn delete<T: Deleteable + SqlParams>(
    client: &tokio_postgres::Client,
    entity: T,
) -> Result<u64, Error> {
    let table_name = T::table_name();
    let where_clause = T::where_clause();

    let sql = format!("DELETE FROM {} WHERE {}", table_name, where_clause);

    let params = entity.params();

    match client.execute(&sql, &params).await {
        Ok(rows_affected) => Ok(rows_affected),
        Err(e) => Err(e),
    }
}

pub async fn get<T: Queryable + FromRow + SqlParams>(
    client: &tokio_postgres::Client,
    entity: T,
) -> Result<T, Error>
{
    let table = T::table_name();
    let columns = T::select_clause().join(", ");
    let where_clause = T::where_clause();

    let sql = format!("SELECT {} FROM {} WHERE {}", columns, table, where_clause);

    let params = entity.params();

    match client.query_one(&sql, &params).await {
        Ok(_row) => Ok(T::from_row(&_row)),
        Err(e) => Err(e),
    }
}

pub async fn get_all<T: Queryable + FromRow + SqlParams>(
    client: &tokio_postgres::Client,
    entity: T,
) -> Result<Vec<T>, Error>
{
    let table = T::table_name();
    let columns = T::select_clause().join(", ");
    let where_clause = T::where_clause();

    let sql = format!("SELECT {} FROM {} WHERE {}", columns, table, where_clause);

    let params = entity.params();

    let rows = client.query(&sql, &params).await?;

    let all_datas: Vec<T> = rows.iter().map(|row| T::from_row(row)).collect();

    Ok(all_datas)
}

pub async fn select<T: Queryable + SqlParams, F>(
    client: &tokio_postgres::Client,
    entity: T,
    to_model: F,
) -> Result<T, Error>
where
    F: Fn(&Row) -> Result<T, Error>,
{
    let table = T::table_name();
    let columns = T::select_clause().join(", ");
    let where_clause = T::where_clause();

    let sql = format!("SELECT {} FROM {} WHERE {}", columns, table, where_clause);

    let params = entity.params();

    match client.query_one(&sql, &params).await {
        Ok(_row) => to_model(&_row),
        Err(e) => Err(e),
    }
}

pub async fn select_all<T: Queryable + SqlParams, F>(
    client: &tokio_postgres::Client,
    entity: T,
    to_model: F,
) -> Result<Vec<T>, Error>
where
    F: Fn(&Row) -> T,
{
    let table = T::table_name();
    let columns = T::select_clause().join(", ");
    let where_clause = T::where_clause();

    let sql = format!("SELECT {} FROM {} WHERE {}", columns, table, where_clause);

    let params = entity.params();

    let rows = client.query(&sql, &params).await?;

    let all_datas: Vec<T> = rows.iter().map(|row| to_model(row)).collect();

    Ok(all_datas)
}

