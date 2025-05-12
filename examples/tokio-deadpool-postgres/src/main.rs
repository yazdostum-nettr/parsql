mod config;
mod models;
mod repository;

use crate::config::DatabaseConfig;
use crate::models::{UserInsert, UserUpdate, UserDelete, UserById, UsersByState, UserStatusQuery, InsertBlog};
use crate::repository::{UserRepository, BlogRepository};
use dotenv::dotenv;
use parsql::deadpool_postgres::Error;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // .env dosyasını yükle (varsa)
    dotenv().ok();

    std::env::set_var("PARSQL_TRACE", "1");
    
    // Bir zaman damgası oluştur - benzersiz e-posta adresleri için
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Zaman hesaplanamadı")
        .as_secs();
    
    // Veritabanı tablolarını oluşturmak için SQL
    const DROP_TABLE_SQL: &str = "DROP TABLE IF EXISTS users";
    const CREATE_TABLE_SQL: &str = r#"
    CREATE TABLE users (
        id BIGSERIAL PRIMARY KEY,
        name TEXT NOT NULL,
        email TEXT NOT NULL,
        state SMALLINT NOT NULL
    )
    "#;
    
    // Blog tablosu için SQL
    const DROP_BLOG_TABLE_SQL: &str = "DROP TABLE IF EXISTS blogs";
    const CREATE_UUID_EXTENSION_SQL: &str = "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"";
    const CREATE_BLOG_TABLE_SQL: &str = r#"
    CREATE TABLE blogs (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
        title TEXT NOT NULL,
        content TEXT,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )
    "#;
    
    // Tabloyu temizle
    const TRUNCATE_TABLE_SQL: &str = "TRUNCATE TABLE users RESTART IDENTITY CASCADE";
    const TRUNCATE_BLOG_TABLE_SQL: &str = "TRUNCATE TABLE blogs";
    
    // Veritabanı konfigürasyonu
    let db_config = DatabaseConfig::from_env();
    println!("Veritabanına bağlanılıyor: {}:{}/{}", db_config.host, db_config.port, db_config.dbname);
    
    // Veritabanı havuzu oluştur
    let pool = db_config.create_pool();
    
    // Veritabanı tablolarını oluştur
    let client = pool.get().await.expect("Havuzdan client alınamadı");
    
    // Users tablosunu oluştur
    client.execute(DROP_TABLE_SQL, &[]).await?;
    println!("Varsa mevcut users tablosu silindi");
    client.execute(CREATE_TABLE_SQL, &[]).await?;
    println!("Users tablosu oluşturuldu");
    client.execute(TRUNCATE_TABLE_SQL, &[]).await?;
    println!("Users tablosu temizlendi");
    
    // Blogs tablosunu oluştur
    client.execute(DROP_BLOG_TABLE_SQL, &[]).await?;
    println!("Varsa mevcut blogs tablosu silindi");
    client.execute(CREATE_UUID_EXTENSION_SQL, &[]).await?;
    println!("UUID extension oluşturuldu");
    client.execute(CREATE_BLOG_TABLE_SQL, &[]).await?;
    println!("Blogs tablosu oluşturuldu");
    client.execute(TRUNCATE_BLOG_TABLE_SQL, &[]).await?;
    println!("Blogs tablosu temizlendi");
    
    // Repository'leri oluştur
    let user_repo = UserRepository::new(pool.clone());
    let blog_repo = BlogRepository::new(pool);
    
    // CRUD işlemlerini göster
    demo_crud_operations(&user_repo, &blog_repo, timestamp).await?;
    
    println!("İşlem başarıyla tamamlandı!");
    Ok(())
}

async fn demo_crud_operations(user_repo: &UserRepository, blog_repo: &BlogRepository, timestamp: u64) -> Result<(), Error> {
    println!("\n=== CRUD İşlemleri Demosu ===");
    
    // 1. Kullanıcı ekleme
    println!("\n1. Kullanıcı Ekleme");
    let new_user = UserInsert::new(
        "Mehmet Yılmaz", 
        &format!("mehmet.yilmaz-{}@exampleone.com", timestamp),
        1 // Aktif kullanıcı (state=1)
    );
    let user_id = user_repo.insert_user(new_user).await?;
    println!("Kullanıcı eklendi, ID: {}", user_id);

    // 2. İkinci kullanıcı ekleme
    println!("\n2. İkinci Kullanıcı Ekleme");
    let second_user = UserInsert::new(
        "Hamza Demir",
        &format!("hamza.demir-{}@exampleone.com", timestamp),
        1 // Aktif kullanıcı (state=1)
    );
    let second_user_id = user_repo.insert_user(second_user).await?;
    println!("İkinci kullanıcı eklendi, ID: {}", second_user_id);

    // 3. Blog ekleme
    println!("\n3. Blog Ekleme");
    let new_blog = InsertBlog::new(
        "Rust ile Veritabanı İşlemleri",
        Some("Bu bir örnek blog içeriğidir. Rust programlama dili ile PostgreSQL veritabanı işlemlerini nasıl yapacağımızı anlatıyor.")
    );
    let blog_id = blog_repo.insert_blog(new_blog).await?;
    println!("Blog eklendi, ID: {}", blog_id);

    // 4. İkinci blog ekleme
    println!("\n4. İkinci Blog Ekleme");
    let second_blog = InsertBlog::new(
        "Rust'ta Asenkron Programlama",
        Some("Rust'ta async/await kullanımı ve tokio runtime ile asenkron programlama örnekleri.")
    );
    let second_blog_id = blog_repo.insert_blog(second_blog).await?;
    println!("İkinci blog eklendi, ID: {}", second_blog_id);
    
    // 2. Kullanıcı güncelleme
    println!("\n2. Kullanıcı Güncelleme");
    let user = user_repo.get_users_by_state(1).await?;
    if let Some(first_user) = user.first() {
        let user_id = first_user.id;
        let update_user = models::UserUpdate::new(
            user_id,
            "Mehmet Yılmaz (Güncellendi)",
            &format!("mehmet.updated-{}@example.com", timestamp)
        );
        let updated = user_repo.update_user(update_user).await?;
        println!("Kullanıcı güncellendi: {}", updated);
        
        // Güncellenen kullanıcıyı getir
        let updated_user = user_repo.get_user_by_id(user_id).await?;
        println!("Güncellenmiş kullanıcı: {:?}", updated_user);
    } else {
        println!("Güncellenecek kullanıcı bulunamadı");
    }
    
    // 3. Özel sorgu ile kullanıcıları getirme (durum bilgisi ile)
    println!("\n3. Özel Sorgu ile Kullanıcıları Getirme");
    let users = user_repo.get_users_with_status(1).await?;
    println!("Kullanıcı durumları:");
    for user in &users {
        println!("  - {}", user);
    }
    
    // 4. Transaction ile kullanıcı ekleme
    println!("\n4. Transaction ile Kullanıcı Ekleme");
    let tx_user = UserInsert::new(
        "Ali Veli", 
        &format!("ali.veli-{}@example.com", timestamp),
        1 // Aktif kullanıcı (state=1)
    );
    let tx_result = user_repo.create_user_with_transaction(tx_user).await?;
    println!("Transaction ile kullanıcı eklendi, kullanıcı ID: {}", tx_result);
    
    // 5. Tüm aktif kullanıcıları listeleme
    println!("\n5. Tüm Aktif Kullanıcıları Listeleme");
    let all_active = user_repo.get_users_by_state(1).await?;
    println!("Toplam {} aktif kullanıcı bulundu:", all_active.len());
    for user in &all_active {
        println!("  - ID: {}, Ad: {}, E-posta: {}", user.id, user.name, user.email);
    }
    
    // 6. (Opsiyonel) Bir kullanıcıyı silme
    if all_active.len() > 1 {
        println!("\n6. Kullanıcı Silme");
        let user_to_delete = all_active.last().unwrap().id;
        let deleted = user_repo.delete_user(user_to_delete).await?;
        println!("Kullanıcı silindi (ID: {}), etkilenen satır sayısı: {}", user_to_delete, deleted);
    }
    
    Ok(())
}

// Demo projesi olduğu için ana veritabanı şemasını oluşturacak yardımcı fonksiyonlar
// Gerçek uygulamalarda migration kullanımı tercih edilmelidir 