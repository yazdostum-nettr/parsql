use std::fmt::Debug;
use tokio_postgres::{NoTls, types::ToSql};
use parsql::tokio_postgres::{CrudOps, Error, Row};

// Makroları parsql::tokio_postgres::macros üzerinden import ediyoruz
use parsql::tokio_postgres::macros::{Queryable, FromRow, SqlParams, Insertable, Updateable, UpdateParams, Deletable};

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
#[select("SELECT id, name, email, CASE WHEN state = 1 THEN 'Aktif' ELSE 'Pasif' END as status FROM users")]
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

// Helper metodları
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

// Ana örnek fonksiyonu
pub async fn run_macro_example() -> Result<(), Error> {
    println!("== Derive Makroları ile CrudOps Extension Örneği ==");
    
    // Veritabanı bağlantısı oluşturma
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=test",
        NoTls,
    ).await?;
    
    // Bağlantıyı arka planda çalıştır
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Bağlantı hatası: {}", e);
        }
    });
    
    // Örnek tablo oluşturma (eğer yoksa)
    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            state SMALLINT NOT NULL
        )",
        &[],
    ).await?;
    
    // Tabloda veri yoksa örnek veri ekleme
    let count = client.query_one("SELECT COUNT(*) FROM users", &[]).await?;
    let count: i64 = count.get(0);
    
    if count == 0 {
        println!("Örnek veri ekleniyor...");
        
        // Extension metot ile veri ekleme (CrudOps trait'in insert metodu)
        let users = vec![
            InsertUser {
                name: "Ali Yılmaz".to_string(),
                email: "ali.yilmaz@example.com".to_string(),
                state: 1,
            },
            InsertUser {
                name: "Ayşe Kaya".to_string(),
                email: "ayse.kaya@example.com".to_string(),
                state: 1,
            },
            InsertUser {
                name: "Mehmet Demir".to_string(),
                email: "mehmet.demir@example.com".to_string(),
                state: 0,
            },
        ];
        
        for user in users {
            // CrudOps trait'inin insert metodunu kullanıyoruz
            let result = client.insert(user).await?;
            println!("Eklenen kayıt ID: {}", result);
        }
    }
    
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
    let users = client.select_all(query, |row| {
        UserWithStatus {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            status: row.get("status"),
        }
    }).await?;
    
    for user in users {
        println!("  - ID: {}, İsim: {}, Durum: {}", user.id, user.name, user.status);
    }
    
    // 5. Kullanıcı silme - CrudOps trait'in delete metodu ile
    println!("\n5. Örnek silme işlemi:");
    
    // Pasif durumdaki kullanıcıyı silme örneği
    let delete_user = DeleteUser { id: 3 };
    let delete_result = client.delete(delete_user).await?;
    println!("Silinen kayıt sayısı: {}", delete_result);
    
    println!("\nDerive Makroları ile CrudOps Extension Örneği tamamlandı.");
    
    Ok(())
} 