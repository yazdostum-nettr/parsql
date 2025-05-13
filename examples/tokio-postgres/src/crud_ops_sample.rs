use parsql::tokio_postgres::{
    macros::{Deletable, FromRow, Insertable, Queryable, SqlParams, UpdateParams, Updateable},
    traits::{CrudOps, FromRow, SqlParams, SqlQuery, UpdateParams},
};
use std::fmt::Debug;
use tokio_postgres::{types::ToSql, Error, NoTls, Row};

// Makroları parsql::macros modülünden import ediyoruz
// use parsql::macros::{
//     Deletable, FromRow as FromRow, Insertable, Queryable, SqlParams as SqlParams,
//     UpdateParams as UpdateParams, Updateable,
// };

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

pub async fn run_crud_ops_example() -> Result<(), Error> {
    println!("== Derive Makroları ile CrudOps Trait Örneği ==");

    // NOT: Veritabanı bağlantısı main.rs üzerinden kurulur ve
    // tablo oluşturma ve örnek veri ekleme işlemleri orada yapılır.
    // Bu örnek yalnızca CrudOps trait'ini göstermek içindir.

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

    // 1. ID'si 1 olan kullanıcıyı getirme - CrudOps trait'in get metodu ile
    println!("\n1. ID'si 1 olan kullanıcıyı getirme:");
    let get_user = GetUser::new(1);
    match client.get(get_user).await {
        Ok(user) => println!("Kullanıcı bulundu: {:?}", user),
        Err(e) => println!("Kullanıcı bulunamadı: {}", e),
    }

    // 2. Aktif kullanıcıları listeleme - CrudOps trait'in get_all metodu ile
    println!("\n2. Aktif kullanıcıları listeleme:");
    let active_users = GetActiveUsers::new();
    let users = client.get_all(active_users).await?;
    println!("Aktif kullanıcı sayısı: {}", users.len());
    for user in users {
        println!("  - {:?}", user);
    }

    // 3. Kullanıcı güncelleme - CrudOps trait'in update metodu ile
    println!("\n3. Kullanıcı güncelleme:");
    let update_user = UpdateUser {
        id: 1,
        name: "Ali Yılmaz (Güncellendi)".to_string(),
        email: "ali.yilmaz.updated@example.com".to_string(),
    };

    let updated = client.update(update_user).await?;
    println!("Güncelleme başarılı: {}", updated);

    // Güncellemeden sonra kullanıcıyı tekrar getirme
    let get_user = GetUser::new(1);
    match client.get(get_user).await {
        Ok(user) => println!("Güncellenmiş kullanıcı: {:?}", user),
        Err(e) => println!("Kullanıcı bulunamadı: {}", e),
    }

    // 4. Özel sorgu ile kullanıcı durumu getirme
    println!("\n4. Özel sorgu ile kullanıcı durumları:");

    // Aktif kullanıcılar için sorgu
    let query = UserStatusQuery { state: 1 };

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

    // 5. Kullanıcı silme - CrudOps trait'in delete metodu ile
    println!("\n5. Örnek silme işlemi:");

    // ID=3 olan kullanıcıyı silme
    let delete_user = DeleteUser { id: 3 };
    let delete_result = client.delete(delete_user).await?;
    println!("Silinen kayıt sayısı: {}", delete_result);

    println!("\nDerive Makroları ile CrudOps Trait Örneği tamamlandı.");

    Ok(())
}
