# parsql-tokio-postgres

Parsql için Tokio PostgreSQL entegrasyon küfesidir. Bu paket, parsql'in tokio-postgres ve deadpool-postgres kütüphaneleriyle asenkron çalışmasını sağlayan API'leri içerir.

## Özellikler

- Asenkron PostgreSQL işlemleri (tokio runtime ile)
- Otomatik SQL sorgu oluşturma
- Güvenli parametre yönetimi
- Generic CRUD işlemleri (get, insert, update, delete)
- Veritabanı satırlarını struct'lara dönüştürme
- Deadpool bağlantı havuzu desteği
- SQL Injection saldırılarına karşı otomatik koruma

## Güvenlik Özellikleri

### SQL Injection Koruması

parsql-tokio-postgres, SQL Injection saldırılarına karşı güvenli bir şekilde tasarlanmıştır:

- Tüm kullanıcı girdileri otomatik olarak parametrize edilir
- PostgreSQL'in "$1, $2, ..." parametrelendirme yapısı otomatik olarak kullanılır
- Makrolar, SQL parametrelerini güvenli bir şekilde işleyerek injection saldırılarına karşı koruma sağlar
- Parametrelerin doğru sırada ve tipte gönderilmesi otomatik olarak yönetilir
- `#[where_clause]` ve diğer SQL bileşenlerinde kullanıcı girdileri her zaman parametrize edilir
- Asenkron bağlamlarda bile güvenlik önlemleri tam olarak korunur

```rust
// SQL injection koruması örneği
#[derive(Queryable, FromRow, SqlParams)]
#[table("users")]
#[where_clause("username = $ AND status = $")]
struct UserQuery {
    username: String,
    status: i32,
}

// Kullanıcı girdisi (potansiyel olarak zararlı) güvenle kullanılır
let query = UserQuery {
    username: kullanici_girdisi, // Bu değer direkt SQL'e eklenmez, parametrize edilir
    status: 1,
};

// Oluşturulan sorgu: "SELECT * FROM users WHERE username = $1 AND status = $2"
// Parametreler güvenli bir şekilde: [kullanici_girdisi, 1] olarak gönderilir
let user = get(&client, query).await?;
```

## Kurulum

Cargo.toml dosyanıza şu şekilde ekleyin:

```toml
[dependencies]
# Tokio PostgreSQL için
parsql = { version = "0.3.2", features = ["tokio-postgres"] }

# veya deadpool bağlantı havuzu ile kullanmak için
parsql = { version = "0.3.2", features = ["deadpool-postgres"] }
```

veya doğrudan bu paketi kullanmak isterseniz:

```toml
[dependencies]
parsql-tokio-postgres = "0.3.2"
parsql-macros = "0.3.2"
tokio-postgres = "0.7"
tokio = { version = "1", features = ["full"] }

# Deadpool kullanmak isterseniz
deadpool-postgres = "0.10"
```

## Temel Kullanım

Bu paket, PostgreSQL veritabanı ile çalışırken **asenkron işlemler** kullanır. Bu, async/await kullanımı gerektirdiği anlamına gelir.

### Bağlantı Kurma

#### Tokio PostgreSQL ile Bağlantı

```rust
use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // PostgreSQL bağlantısı oluşturma
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
    
    // Örnek tablo oluşturma
    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            state SMALLINT NOT NULL
        )",
        &[],
    ).await?;
    
    // ...
    
    Ok(())
}
```

#### Deadpool PostgreSQL ile Bağlantı Havuzu

```rust
use deadpool_postgres::{Config, Client, Pool};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Deadpool konfigürasyonu
    let mut cfg = Config::new();
    cfg.host = Some("localhost".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("postgres".to_string());
    cfg.dbname = Some("test".to_string());
    
    // Bağlantı havuzu oluşturma
    let pool = cfg.create_pool(None, NoTls)?;
    
    // Havuzdan bağlantı alma
    let client: Client = pool.get().await?;
    
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
    tokio_postgres::{FromRow, SqlParams, get},
};
use tokio_postgres::{types::ToSql, Row};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=test",
        NoTls,
    ).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Bağlantı hatası: {}", e);
        }
    });
    
    // Kullanımı
    let get_user = GetUser::new(1);
    let get_result = get(&client, get_user).await?;
    
    println!("Kullanıcı: {:?}", get_result);
    Ok(())
}
```

### Veri Ekleme (Insert) İşlemi

```rust
use parsql::{
    core::Insertable,
    macros::{Insertable, SqlParams},
    tokio_postgres::{SqlParams, insert},
};
use tokio_postgres::types::ToSql;

#[derive(Insertable)]
#[table("users")]
pub struct InsertUser {
    pub name: String,
    pub email: String,
    pub state: i16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=test",
        NoTls,
    ).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Bağlantı hatası: {}", e);
        }
    });
    
    let insert_user = InsertUser {
        name: "Ali".to_string(),
        email: "ali@parsql.com".to_string(),
        state: 1,
    };
    
    let insert_result = insert(&client, insert_user).await?;
    println!("Eklenen kayıt ID: {}", insert_result);
    
    Ok(())
}
```

### Veri Güncelleme (Update) İşlemi

```rust
use parsql::{
    core::Updateable,
    macros::{UpdateParams, Updateable},
    tokio_postgres::{UpdateParams, update},
};
use tokio_postgres::types::ToSql;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=test",
        NoTls,
    ).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Bağlantı hatası: {}", e);
        }
    });
    
    let update_user = UpdateUser {
        id: 1,
        name: String::from("Ali Güncellendi"),
        email: String::from("ali.updated@gmail.com"),
        state: 2,
    };
    
    let result = update(&client, update_user).await?;
    println!("Güncellenen kayıt sayısı: {}", result);
    
    Ok(())
}
```

### Veri Silme (Delete) İşlemi

```rust
use parsql::{
    core::Deletable,
    macros::{Deletable, SqlParams},
    tokio_postgres::{SqlParams, delete},
};
use tokio_postgres::types::ToSql;

#[derive(Deletable, SqlParams)]
#[table("users")]
#[where_clause("id = $")]
pub struct DeleteUser {
    pub id: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=test",
        NoTls,
    ).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Bağlantı hatası: {}", e);
        }
    });
    
    let delete_user = DeleteUser { id: 1 };
    let result = delete(&client, delete_user).await?;
    
    println!("Silinen kayıt sayısı: {}", result);
    Ok(())
}
```

## Deadpool ile Kullanım

Deadpool bağlantı havuzu ile parsql kullanmak için öncelikle cargo.toml dosyanızda "deadpool-postgres" özelliğini etkinleştirmeniz gerekiyor. Sonrasında, havuzdan aldığınız istemci üzerinde parsql fonksiyonlarını kullanabilirsiniz.

```rust
use deadpool_postgres::{Config, Pool};
use tokio_postgres::NoTls;
use parsql::tokio_postgres::{get, insert};

// Havuz bağlantısıyla get işlemi
async fn fetch_user(pool: &Pool, user_id: i64) -> Result<GetUser, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let get_user = GetUser::new(user_id);
    let result = get(&client, get_user).await?;
    Ok(result)
}

// Havuz bağlantısıyla insert işlemi
async fn create_user(pool: &Pool, user: InsertUser) -> Result<i64, Box<dyn std::error::Error>> {
    let client = pool.get().await?;
    let result = insert(&client, user).await?;
    Ok(result)
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

Bu, tokio-postgres için oluşturulan tüm sorguları konsola yazdıracaktır.

## Performans İpuçları

1. **Prepared Statements**: tokio-postgres, sorguları prepared statement olarak çalıştırır ve parsql bu özelliği kullanır, bu SQL enjeksiyonlarına karşı korunmanıza yardımcı olur.

2. **Bağlantı Havuzu**: Yüksek yüklü uygulamalarda deadpool-postgres kullanımı daha iyi performans sağlar.

3. **Asenkron İşlemler**: tokio-postgres ile işlemlerinizi asenkron olarak çalıştırarak, uygulamanızın daha verimli çalışmasını sağlayabilirsiniz.

## Hata Yakalama

Tokio-postgres işlemleri sırasında oluşabilecek hataları yakalamak ve işlemek için Rust'ın `Result` mekanizmasını kullanın:

```rust
match get(&client, get_user).await {
    Ok(user) => println!("Kullanıcı bulundu: {:?}", user),
    Err(e) => eprintln!("Hata oluştu: {}", e),
}
```

## Tam Örnek Proje

Tam bir örnek proje için parsql ana deposundaki [examples/tokio-postgres](../examples/tokio-postgres) dizinine bakabilirsiniz.
