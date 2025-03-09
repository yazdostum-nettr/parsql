# parsql-sqlite

Parsql için SQLite entegrasyon küfesidir. Bu paket, parsql'in SQLite veritabanlarıyla çalışmasını sağlayan senkron API'leri içerir.

## Özellikler

- Senkron SQLite işlemleri
- Otomatik SQL sorgu oluşturma
- Güvenli parametre yönetimi
- Generic CRUD işlemleri (get, insert, update)
- Veritabanı satırlarını struct'lara dönüştürme
- SQL Injection saldırılarına karşı otomatik koruma

## Güvenlik Özellikleri

### SQL Injection Koruması

parsql-sqlite, SQL Injection saldırılarına karşı güvenli bir şekilde tasarlanmıştır:

- Tüm kullanıcı girdileri otomatik olarak parametrize edilir
- SQLite'ın "?" parametrelendirme yapısı otomatik olarak kullanılır
- Makrolar, SQL parametrelerini güvenli bir şekilde işleyerek injection saldırılarına karşı koruma sağlar
- Parametrelerin doğru sırada ve tipte gönderilmesi otomatik olarak yönetilir
- `#[where_clause]` ve diğer SQL bileşenlerinde kullanıcı girdileri her zaman parametrize edilir

```rust
// SQL injection koruması örneği
#[derive(Queryable, FromRow, SqlParams)]
#[table("users")]
#[where_clause("username = ? AND status = ?")]
struct UserQuery {
    username: String,
    status: i32,
}

// Kullanıcı girdisi (potansiyel olarak zararlı) güvenle kullanılır
let query = UserQuery {
    username: kullanici_girdisi, // Bu değer direkt SQL'e eklenmez, parametrize edilir
    status: 1,
};

// Oluşturulan sorgu: "SELECT * FROM users WHERE username = ? AND status = ?"
// Parametreler güvenli bir şekilde: [kullanici_girdisi, 1] olarak gönderilir
let user = get(&conn, query)?;
```

## Kurulum

Cargo.toml dosyanıza şu şekilde ekleyin:

```toml
[dependencies]
parsql = { version = "0.3.0", features = ["sqlite"] }
```

veya doğrudan bu paketi kullanmak isterseniz:

```toml
[dependencies]
parsql-sqlite = "0.3.0"
parsql-macros = "0.3.0"
```

## Temel Kullanım

Bu paket, SQLite veritabanı ile çalışırken **senkron işlemler** kullanır. Bu, async/await kullanımı gerektirmediği anlamına gelir.

### Bağlantı Kurma

```rust
use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // SQLite bağlantısı oluşturma
    let conn = Connection::open("veritabani.db")?;
    
    // Örnek tablo oluşturma
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            state INTEGER NOT NULL
        )",
        [],
    )?;
    
    // ...
    
    Ok(())
}
```

## CRUD İşlemleri

### Veri Okuma (Get) İşlemi

```rust
use parsql::{
    core::Queryable,
    macros::{FromRow, Queryable, SqlParams},
    sqlite::{FromRow, SqlParams, get},
};
use rusqlite::{types::ToSql, Row};

#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[where_clause("id = $")]
pub struct GetUser {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

impl GetUser {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            name: Default::default(),
            email: Default::default(),
            state: Default::default(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("veritabani.db")?;
    
    // Kullanımı
    let get_user = GetUser::new(1);
    let get_result = get(&conn, get_user)?;
    
    println!("Kullanıcı: {:?}", get_result);
    Ok(())
}
```

### Veri Ekleme (Insert) İşlemi

```rust
use parsql::{
    core::Insertable,
    macros::{Insertable, SqlParams},
    sqlite::{SqlParams, insert},
};
use rusqlite::types::ToSql;

#[derive(Insertable)]
#[table("users")]
pub struct InsertUser {
    pub name: String,
    pub email: String,
    pub state: i16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("veritabani.db")?;
    
    let insert_user = InsertUser {
        name: "Ali".to_string(),
        email: "ali@parsql.com".to_string(),
        state: 1,
    };
    
    let insert_result = insert(&conn, insert_user)?;
    println!("Eklenen kayıt ID: {}", insert_result);
    
    Ok(())
}
```

### Veri Güncelleme (Update) İşlemi

```rust
use parsql::{
    core::Updateable,
    macros::{UpdateParams, Updateable},
    sqlite::{UpdateParams, update},
};
use rusqlite::types::ToSql;

#[derive(Updateable, UpdateParams)]
#[table("users")]
#[update("name, email")]
#[where_clause("id = $")]
pub struct UpdateUser {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("veritabani.db")?;
    
    let update_user = UpdateUser {
        id: 1,
        name: String::from("Ali Güncellendi"),
        email: String::from("ali.updated@gmail.com"),
        state: 2,
    };
    
    let result = update(&conn, update_user)?;
    println!("Güncellenen kayıt sayısı: {}", result);
    
    Ok(())
}
```

### Veri Silme (Delete) İşlemi

```rust
use parsql::{
    core::Deletable,
    macros::{Deletable, SqlParams},
    sqlite::{SqlParams, delete},
};
use rusqlite::types::ToSql;

#[derive(Deletable, SqlParams)]
#[table("users")]
#[where_clause("id = $")]
pub struct DeleteUser {
    pub id: i64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("veritabani.db")?;
    
    let delete_user = DeleteUser { id: 1 };
    let result = delete(&conn, delete_user)?;
    
    println!("Silinen kayıt sayısı: {}", result);
    Ok(())
}
```

## Gelişmiş Özellikler

### Join Kullanımı

```rust
#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[select("users.id, users.name, posts.title as post_title")]
#[join("LEFT JOIN posts ON users.id = posts.user_id")]
#[where_clause("users.id = $")]
pub struct UserWithPosts {
    pub id: i64,
    pub name: String,
    pub post_title: Option<String>,
}
```

### Gruplama ve Sıralama

```rust
#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[select("state, COUNT(*) as user_count")]
#[group_by("state")]
#[order_by("user_count DESC")]
#[having("COUNT(*) > 5")]
pub struct UserStats {
    pub state: i16,
    pub user_count: i64,
}
```

### Özel Select İfadeleri

```rust
#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[select("id, name, email, CASE WHEN state = 1 THEN 'Aktif' ELSE 'Pasif' END as status")]
#[where_clause("id = $")]
pub struct UserWithStatus {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub status: String,
}
```

## SQL Sorgularını İzleme

Oluşturulan SQL sorgularını görmek için `PARSQL_TRACE` çevre değişkenini ayarlayabilirsiniz:

```sh
PARSQL_TRACE=1 cargo run
```

Bu, SQLite için oluşturulan tüm sorguları konsola yazdıracaktır.

## Performans İpuçları

1. **İndeksleme**: SQLite sorgularınızın performansını artırmak için, sıkça sorguladığınız sütunlarda indeks oluşturun.

   ```sql
   CREATE INDEX idx_users_email ON users(email);
   ```

2. **Toplu İşlemler**: Birden çok insert, update veya delete işlemi yaparken, işlemleri bir transaction içinde yapın:

   ```rust
   conn.execute("BEGIN TRANSACTION", [])?;
   // İşlemlerinizi burada yapın
   conn.execute("COMMIT", [])?;
   ```

3. **Prepared Statements**: Parsql zaten arkada prepared statement kullanır, bu SQL enjeksiyon saldırılarına karşı korunmanıza yardımcı olur.

## Hata Yakalama

SQLite işlemleri sırasında oluşabilecek hataları yakalamak ve işlemek için Rust'ın `Result` mekanizmasını kullanın:

```rust
match get(&conn, get_user) {
    Ok(user) => println!("Kullanıcı bulundu: {:?}", user),
    Err(e) => eprintln!("Hata oluştu: {}", e),
}
```

## Tam Örnek Proje

Tam bir örnek proje için parsql ana deposundaki [examples/sqlite](../examples/sqlite) dizinine bakabilirsiniz.

