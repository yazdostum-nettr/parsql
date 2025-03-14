use std::default;

use example_parsql_postgres::{
    insert_sample::{InsertComment, InsertPost},
    DeleteUser, InsertUser, SelectUser, SelectUserWithPosts, UpdateUser, UserStateStats, UserPostStats,
    transaction_sample::{transaction_with_crud_ops, transaction_with_helper_functions, transaction_with_rollback, complex_transaction_example, transaction_with_delete},
    pagination_sample::{run_pagination_examples, run_derive_pagination_examples},
};
use parsql::postgres::{get_all, insert, select, update, delete};
use postgres::{Client, NoTls};

fn init_connection() -> Client {
    let mut client = Client::connect(
        "host=localhost user=myuser password=mypassword dbname=sample_db",
        NoTls,
    )
    .expect("Postgresql ile bağlantı aşamasında bir hata oluştu!");

    // Tabloları oluştur ve örnek veriler ekle,
    let _ = client.batch_execute(
        "
    DROP TABLE IF EXISTS comments;
    DROP TABLE IF EXISTS posts;
    DROP TABLE IF EXISTS users;

    CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        name TEXT NOT NULL,
        email TEXT NOT NULL,
        state SMALLINT NOT NULL DEFAULT 1
    );

    CREATE TABLE IF NOT EXISTS posts (
        id SERIAL PRIMARY KEY,
        user_id INT NOT NULL REFERENCES users(id),
        content TEXT NOT NULL,
        state SMALLINT NOT NULL DEFAULT 1,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TABLE IF NOT EXISTS comments (
        id SERIAL PRIMARY KEY,
        post_id INT NOT NULL REFERENCES posts(id),
        content TEXT NOT NULL,
        state SMALLINT NOT NULL DEFAULT 1,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

    -- Temel örnek veriler
    INSERT INTO users (name, email, state) VALUES 
        ('Admin', 'admin@example.com', 1),
        ('User1', 'user1@example.com', 1),
        ('User2', 'user2@example.com', 2);

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
        ('User3', 'user3@example.com', 1),
        ('User4', 'user4@example.com', 1),
        ('User5', 'user5@example.com', 2),
        ('User6', 'user6@example.com', 1),
        ('User7', 'user7@example.com', 1),
        ('User8', 'user8@example.com', 2),
        ('User9', 'user9@example.com', 1),
        ('User10', 'user10@example.com', 1),
        ('User11', 'user11@example.com', 2),
        ('User12', 'user12@example.com', 1),
        ('User13', 'user13@example.com', 1),
        ('User14', 'user14@example.com', 2),
        ('User15', 'user15@example.com', 1),
        ('User16', 'user16@example.com', 1),
        ('User17', 'user17@example.com', 2),
        ('User18', 'user18@example.com', 1),
        ('User19', 'user19@example.com', 1),
        ('User20', 'user20@example.com', 2);
    ",
    ).expect("Tablo oluşturma işlemi başarısız oldu!");
    
    client
}

fn main() {

    /*
    # Unix/Linux/MacOS için
    PARSQL_TRACE=1 cargo run

    # Windows PowerShell için
    $env:PARSQL_TRACE=1; cargo run

    # Windows CMD için
    set PARSQL_TRACE=1 && cargo run
    */

    std::env::set_var("PARSQL_TRACE", "1");

    let mut db = init_connection();

    let insert_user = InsertUser {
        name: "Ali".to_string(),
        email: "alice@parsql.com".to_string(),
        state: 1_i16,
    };

    let insert_result = insert(&mut db, insert_user);
    println!("Insert result: {:?}", insert_result);

    let insert_post = InsertPost {
        user_id: 1_i32,
        content: "Post 1".to_string(),
        state: 1_i16,
    };

    let insert_result = insert(&mut db, insert_post);
    println!("Insert result: {:?}", insert_result);

    let insert_comment = InsertComment {
        post_id: 1_i32,
        content: "Comment 1".to_string(),
        state: 1,
    };

    let insert_result = insert(&mut db, insert_comment);
    println!("Insert result: {:?}", insert_result);

    let update_user = UpdateUser {
        id: 1,
        name: String::from("Ali"),
        email: String::from("ali@gmail.com"),
        state: 2,
    };

    let result = update(&mut db, update_user);

    println!("Update result: {:?}", result);

    let select_user = SelectUser {
        id: 1,
        name: default::Default::default(),
        email: default::Default::default(),
        state: default::Default::default(),
    };

    let select_result = select(&mut db, select_user, |row| {
        Ok(SelectUser {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            state: row.get(3),
        })
    });

    println!("Select result: {:?}", select_result);

    let select_user_with_posts = SelectUserWithPosts::new(1_i32);

    let get_user_with_posts = get_all(&mut db, &select_user_with_posts);

    println!("Get user with posts: {:?}", get_user_with_posts);

    // Kullanıcı durumu istatistikleri (HAVING ile)
    let user_state_stats = UserStateStats::new(0);
    let stats_result = get_all(&mut db, &user_state_stats);
    println!("User state stats: {:?}", stats_result);

    // Kullanıcı gönderi istatistikleri (HAVING ile)
    let user_post_stats = UserPostStats::new(0);
    let post_stats_result = get_all(&mut db, &user_post_stats);
    println!("User post stats: {:?}", post_stats_result);

    // 3. Silme İşlemleri
    println!("\n3. Silme İşlemleri:");
    
    // Önce bu kullanıcının gönderilerini siliyoruz
    let user_id = 3_i32;
    let posts_deleted = db.execute("DELETE FROM comments WHERE post_id IN (SELECT id FROM posts WHERE user_id = $1)", &[&user_id]).expect("Yorumları silme hatası");
    println!("Silinen yorum sayısı: {}", posts_deleted);
    
    let posts_deleted = db.execute("DELETE FROM posts WHERE user_id = $1", &[&user_id]).expect("Gönderileri silme hatası");
    println!("Silinen gönderi sayısı: {}", posts_deleted);
    
    // Şimdi kullanıcıyı silebiliriz
    let user_to_delete = DeleteUser { id: user_id };
    let deleted_count = delete(&mut db, user_to_delete).expect("Kullanıcı silme hatası");
    println!("Silinen kullanıcı sayısı: {}", deleted_count);

    // 4. Transaction İşlemleri
    println!("\n4. Transaction İşlemleri:");
    
    // CrudOps trait'inin Transaction için kullanımı
    transaction_with_crud_ops(&mut db).expect("Transaction ile işlemler başarısız oldu");
    
    // Yardımcı fonksiyonlarla transaction kullanımı
    transaction_with_helper_functions(&mut db).expect("Transaction yardımcı fonksiyonları başarısız oldu");
    
    // Hata durumunda rollback örneği
    transaction_with_rollback(&mut db).expect("Rollback örneği başarısız oldu");
    
    // Karmaşık transaction örneği
    complex_transaction_example(&mut db).expect("Karmaşık transaction örneği başarısız oldu");
    
    // Silme işlemi içeren transaction örneği
    transaction_with_delete(&mut db).expect("Silme işlemi içeren transaction örneği başarısız oldu");
    
    // 5. Sayfalama Örnekleri
    run_pagination_examples(&mut db).expect("Sayfalama örnekleri başarısız oldu");

    // 6. Derive Makrosu ile Sayfalama Örnekleri
    run_derive_pagination_examples(&mut db).expect("Derive makro ile sayfalama örnekleri başarısız oldu");
}


