use parsql::sqlite::crud_ops::get;
use parsql::sqlite::traits::CrudOps;
use parsql::sqlite::{delete, insert, update};
use parsql::sqlite::{
    macros::{Deletable, Insertable, Queryable, SqlParams, UpdateParams, Updateable},
    traits::{FromRow, SqlParams, SqlQuery, UpdateParams},
};
use rusqlite::{Connection, Error, Result};

// Modülleri import et
mod models;
mod pagination_sample;

// Modüllerden yapıları import et
use models::stats::UserPostStatsAdvanced;
use models::user::{DeleteUser, GetUser, GetUserByName, InsertUser, UpdateUser};
use pagination_sample::{run_derive_pagination_examples, run_pagination_examples};

fn main() {
    // PARSQL_TRACE çevre değişkenini ayarla
    std::env::set_var("PARSQL_TRACE", "1");

    let conn = Connection::open("sqlite_db.db3").unwrap();

    // Tabloları oluştur ve örnek veriler ekle
    let _ = conn
        .execute_batch(
            "
        DROP TABLE IF EXISTS comments;
        DROP TABLE IF EXISTS posts;
        DROP TABLE IF EXISTS users;

        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT, 
            name TEXT NOT NULL, 
            email TEXT NOT NULL, 
            state INTEGER NOT NULL DEFAULT 1
        );
        
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            content TEXT NOT NULL,
            state INTEGER NOT NULL DEFAULT 1,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
        
        CREATE TABLE IF NOT EXISTS comments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            post_id INTEGER NOT NULL,
            content TEXT NOT NULL,
            state INTEGER NOT NULL DEFAULT 1,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (post_id) REFERENCES posts(id)
        );
        
        -- Örnek veriler
        INSERT INTO users (name, email, state) VALUES 
            ('admin', 'admin@example.com', 1),
            ('user1', 'user1@example.com', 1),
            ('user2', 'user2@example.com', 2);
            
        INSERT INTO posts (user_id, content, state) VALUES
            (1, 'Admin post 1', 1),
            (1, 'Admin post 2', 1),
            (2, 'User1 post 1', 1),
            (3, 'User2 post 1', 2);
            
        INSERT INTO comments (post_id, content, state) VALUES
            (1, 'Comment on admin post 1', 1),
            (1, 'Another comment on admin post 1', 1),
            (2, 'Comment on admin post 2', 1),
            (3, 'Comment on User1 post', 1);
            
        -- Sayfalama örnekleri için daha fazla test verisi
        INSERT INTO users (name, email, state) VALUES 
            ('user3', 'user3@example.com', 1),
            ('user4', 'user4@example.com', 1),
            ('user5', 'user5@example.com', 2),
            ('user6', 'user6@example.com', 1),
            ('user7', 'user7@example.com', 1),
            ('user8', 'user8@example.com', 2),
            ('user9', 'user9@example.com', 1),
            ('user10', 'user10@example.com', 1),
            ('user11', 'user11@example.com', 2),
            ('user12', 'user12@example.com', 1),
            ('user13', 'user13@example.com', 1),
            ('user14', 'user14@example.com', 2),
            ('user15', 'user15@example.com', 1),
            ('user16', 'user16@example.com', 1),
            ('user17', 'user17@example.com', 2),
            ('user18', 'user18@example.com', 1),
            ('user19', 'user19@example.com', 1),
            ('user20', 'user20@example.com', 2);
    ",
        )
        .expect("Tablo oluşturma işlemi başarısız oldu!");

    let insert_usert = InsertUser {
        name: "Ali".to_string(),
        email: "ali@parsql.com".to_string(),
        state: 1,
    };

    let insert_result = insert::<InsertUser, i64>(&conn, insert_usert);
    let last_id = conn.last_insert_rowid();
    println!(
        "işlem başarıyla tamamlandı! Sonuç: {:?}, Son eklenen ID: {}",
        insert_result, last_id
    );

    let update_user = UpdateUser {
        id: last_id,
        name: String::from("Ali"),
        email: String::from("ali@gmail.com"),
        state: 2,
    };

    let update_result = update(&conn, update_user);
    println!("Update result: {:?}", update_result);

    let get_user = GetUser {
        id: 1,
        name: Default::default(),
        email: Default::default(),
        state: Default::default(),
    };

    let get_user_result = conn.fetch(&get_user);

    println!("get user result: {:?}", get_user_result);

    // SQL Injection Denemesi
    let malicious_name = "' OR '1'='1";

    let get_user = GetUserByName {
        id: 0,
        name: malicious_name.to_string(),
        email: String::new(),
        state: 0,
    };

    match get(&conn, &get_user) {
        Ok(user) => println!("Bulunan kullanıcı: {:?}", user),
        Err(e) => println!("Hata: {}", e),
    }

    // parsql için isim ile sorgulama örneği
    let get_user = GetUserByName {
        id: 0,
        name: "admin".to_string(),
        email: String::new(),
        state: 0,
    };

    match conn.fetch(&get_user) {
        Ok(user) => println!("Bulunan kullanıcı: {:?}", user),
        Err(e) => println!("Hata: {}", e),
    }

    // UserPostStatsAdvanced örneği
    let stats_query = UserPostStatsAdvanced::new(0);

    match get(&conn, &stats_query) {
        Ok(stats) => println!("Gelişmiş kullanıcı-gönderi istatistikleri: {:?}", stats),
        Err(e) => println!("İstatistik sorgulama hatası: {}", e),
    }

    // Tüm istatistikleri getirme örneği
    println!("\nTüm gelişmiş kullanıcı-gönderi istatistikleri:");

    // DELETE işlemi (doğrudan SQL sorgusu ile)
    let user_id_to_delete = 3;
    let delete_user = DeleteUser {
        id: user_id_to_delete,
    };
    let deleted_rows = delete(&conn, delete_user);
    println!("Silinen satır sayısı: {:?}", deleted_rows);

    // 5. Sayfalama Örnekleri
    run_pagination_examples(&conn).expect("Manuel sayfalama örnekleri başarısız oldu");

    // 6. Derive Macro ile Sayfalama Örnekleri
    run_derive_pagination_examples(&conn)
        .expect("Derive macro ile sayfalama örnekleri başarısız oldu");
}
