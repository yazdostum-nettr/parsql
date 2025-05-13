use parsql::tokio_postgres::{
    macros::{Deletable, FromRow, Insertable, Queryable, SqlParams, UpdateParams, Updateable},
    traits::{CrudOps, FromRow, SqlParams, SqlQuery, UpdateParams},
    Error, Row,
};
use std::fmt::Debug;
use tokio_postgres::{types::ToSql, NoTls};

// Kullanıcı tablosundan veri almak için modeli tanımlıyoruz
#[derive(Debug, Queryable, FromRow, SqlParams)]
#[table("users")]
#[where_clause("id = $")]
pub struct GetUser {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

// Yeni kullanıcı eklemek için veri modeli
#[derive(Insertable, SqlParams)]
#[table("users")]
pub struct InsertUser {
    pub name: String,
    pub email: String,
    pub state: i16,
}

// Kullanıcı güncellemek için veri modeli
#[derive(Updateable, UpdateParams)]
#[table("users")]
#[update("name, email")]
#[where_clause("id = $")]
pub struct UpdateUser {
    pub id: i64,
    pub name: String,
    pub email: String,
}

// Kullanıcı silmek için veri modeli
#[derive(Deletable, SqlParams)]
#[table("users")]
#[where_clause("id = $")]
pub struct DeleteUser {
    pub id: i64,
}

// Aktif kullanıcıları almak için veri modeli
#[derive(Debug, Queryable, FromRow, SqlParams)]
#[table("users")]
#[where_clause("state = $")]
#[order_by("name ASC")]
pub struct GetActiveUsers {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

// Özel sorgu için model
#[derive(Queryable, SqlParams)]
#[table("users")]
#[select("id, name, email, CASE WHEN state = 1 THEN 'Aktif' ELSE 'Pasif' END as status")]
#[where_clause("state = $")]
pub struct UserStatusQuery {
    pub state: i16,
}

// Özel sorguda kullanılacak sonuç modeli
#[derive(Debug, FromRow)]
pub struct UserWithStatus {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub status: String,
}

impl GetUser {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            name: String::new(),
            email: String::new(),
            state: 0,
        }
    }
}

impl InsertUser {
    pub fn new(name: String, email: String, state: i16) -> Self {
        Self { name, email, state }
    }
}

impl UpdateUser {
    pub fn new(id: i64, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}

impl DeleteUser {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
}

impl GetActiveUsers {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
            email: String::new(),
            state: 1, // Aktif kullanıcılar için state=1
        }
    }
}

impl UserStatusQuery {
    pub fn new(state: i16) -> Self {
        Self { state }
    }
}

pub async fn run_macro_example() -> Result<(), Error> {
    println!("== Derive Makroları ile CrudOps Örneği ==");

    // NOT: Veritabanı bağlantısı main.rs üzerinden kurulur ve
    // tablo oluşturma ve örnek veri ekleme işlemleri orada yapılır.
    // Bu örnek yalnızca makroları göstermek içindir.

    // Veritabanı bağlantısı oluşturma
    let (client, connection) = tokio_postgres::connect(
        &dotenvy::var("DATABASE_URL").unwrap_or_else(|_| {
            format!(
                "host={} port={} user={} password={} dbname={}",
                dotenvy::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
                dotenvy::var("DB_PORT").unwrap_or_else(|_| "5432".to_string()),
                dotenvy::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()),
                dotenvy::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string()),
                dotenvy::var("DB_NAME").unwrap_or_else(|_| "postgres".to_string())
            )
        }),
        NoTls,
    )
    .await?;

    // Bağlantıyı arka planda çalıştır
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Bağlantı hatası: {}", e);
        }
    });

    // Makro ile oluşturulan sorgular
    println!("\nMakro kullanımı örnekleri:");

    // 1. Tekli veri getirme - get metodu
    println!("\n1. ID'si 1 olan kullanıcıyı getirme:");
    let get_user = GetUser::new(1);
    match client.get(get_user).await {
        Ok(user) => println!("Kullanıcı bulundu: {:?}", user),
        Err(e) => println!("Kullanıcı bulunamadı: {}", e),
    }

    // 2. Çoklu veri getirme - get_all metodu
    println!("\n2. Aktif kullanıcıları listeleme:");
    let active_users = GetActiveUsers::new();
    let users = client.get_all(active_users).await?;
    println!("Aktif kullanıcı sayısı: {}", users.len());
    for user in users {
        println!("  - {:?}", user);
    }

    // 3. Veri güncelleme - update metodu
    println!("\n3. Kullanıcı güncelleme:");
    let update_user = UpdateUser::new(
        1,
        "Zeynep Kaya (Güncellendi)".to_string(),
        "zeynep.updated@example.com".to_string(),
    );

    let updated = client.update(update_user).await?;
    println!("Güncelleme başarılı: {}", updated);

    // Özel sorgu ile veri getirme
    println!("\n4. Özel sorgu ile kullanıcı durumları:");

    // Aktif kullanıcılar için sorgu
    let query = UserStatusQuery::new(1);

    // select_all ile sorgu çalıştırma
    let users = client
        .select_all(query, |row| UserWithStatus {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            status: row.get("status"),
        })
        .await?;

    for user in users {
        println!(
            "  - ID: {}, İsim: {}, Durum: {}",
            user.id, user.name, user.status
        );
    }

    // 5. Kullanıcı silme
    println!("\n5. Kullanıcı silme:");
    let delete_user = DeleteUser::new(3);
    let delete_result = client.delete(delete_user).await?;
    println!("Silinen kayıt sayısı: {}", delete_result);

    println!("\nMakro örneği tamamlandı.");

    Ok(())
}
