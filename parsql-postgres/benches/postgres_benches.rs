use criterion::{black_box, criterion_group, criterion_main, Criterion};
use parsql_core::Insertable;
use parsql_macros::Insertable;
use parsql_postgres::{insert, SqlParams};
use postgres::{types::ToSql, Client, NoTls};

#[derive(Insertable)]
#[table_name("users")]
pub struct InsertUser {
    pub name: String,
    pub email: String,
    pub state: i16,
}

fn init_connection() -> Client {
    let mut client = Client::connect(
        "host=localhost user=myuser password=mypassword dbname=sample_db",
        NoTls,
    )
    .expect("Postgresql ile bağlantı aşamasında bir hata oluştu!");

    let _ = client.batch_execute(
        "CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        name VARCHAR(100) NOT NULL,
        email VARCHAR(255) NOT NULL,
        state INTEGER
    );",
    );

    client
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut db = init_connection();

    c.bench_function("postgres-insert user", |b| {
        b.iter(|| {
            let user = black_box(InsertUser {
                name: "SampleName".to_string(),
                email: "SampleEmail".to_string(),
                state: 1,
            });
            let _ = insert(&mut db, user);
        })
    });
}

criterion_group! {name = benches; config = Criterion::default().measurement_time(std::time::Duration::from_secs(20)).significance_level(0.1).sample_size(10000); targets = criterion_benchmark}
criterion_main!(benches);
