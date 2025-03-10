# parsql-deadpool-postgres

Parsql için Deadpool PostgreSQL entegrasyon küfesidir. Bu paket, parsql'in deadpool-postgres kütüphanesi ile asenkron bağlantı havuzu yönetimini destekleyen API'leri içerir.

## Özellikler

- Deadpool ile PostgreSQL bağlantı havuzu yönetimi
- Asenkron PostgreSQL işlemleri (tokio runtime ile)
- Otomatik SQL sorgu oluşturma
- Güvenli parametre yönetimi
- Generic CRUD işlemleri (get, insert, update, delete)
- Veritabanı satırlarını struct'lara dönüştürme
- Özel satır dönüşümleri
- SQL Injection saldırılarına karşı otomatik koruma
- Transaction desteği

## Güvenlik Özellikleri

### SQL Injection Koruması

parsql-deadpool-postgres, SQL Injection saldırılarına karşı güvenli bir şekilde tasarlanmıştır:

- Tüm kullanıcı girdileri otomatik olarak parametrize edilir
- PostgreSQL'in "$1, $2, ..." parametrelendirme yapısı otomatik olarak kullanılır
- Makrolar, SQL parametrelerini güvenli bir şekilde işleyerek injection saldırılarına karşı koruma sağlar
- Parametrelerin doğru sırada ve tipte gönderilmesi otomatik olarak yönetilir
- `#[where_clause]` ve diğer SQL bileşenlerinde kullanıcı girdileri her zaman parametrize edilir
- Bağlantı havuzu kullanırken bile güvenlik önlemleri tam olarak korunur

```rust
// SQL injection koruması örneği
#[derive(Queryable)]
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
let user = get(&pool, &query).await?;
```

## Kurulum

Cargo.toml dosyanıza şu şekilde ekleyin:

```toml
[dependencies]
# Deadpool PostgreSQL için
parsql-deadpool-postgres = "0.3.2"
parsql-macros = "0.3.2"
deadpool-postgres = "0.14"
tokio-postgres = "0.7"
tokio = { version = "1", features = ["full"] }
```

## Temel Kullanım

Bu paket, PostgreSQL veritabanı ile çalışırken **asenkron işlemler** ve **bağlantı havuzu yönetimi** kullanır. Bu, async/await kullanımı gerektirdiği anlamına gelir.

### Bağlantı Havuzu Oluşturma

```rust
use deadpool_postgres::{Config, Runtime};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // PostgreSQL bağlantı havuzu oluşturma
    let mut cfg = Config::new();
    cfg.host = Some("localhost".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("postgres".to_string());
    cfg.dbname = Some("test".to_string());
    
    // Bağlantı havuzunu oluşturma
    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
    
    // ... havuz kullanımı ...
    
    Ok(())
}
```

### Modelleri Tanımlama

Veritabanı CRUD işlemleri için, ilgili makrolarla işaretlenmiş model struct'ları tanımlayın:

```rust
use parsql_deadpool_postgres::macros::{Insertable, Updateable, Queryable, Deletable};

// Eklemek için model
#[derive(Insertable)]
#[table("users")]
struct UserInsert {
    name: String,
    email: String,
    active: bool,
}

// Güncellemek için model
#[derive(Updateable)]
#[table("users")]
#[update("name, email, active")]
#[where_clause("id = $")]
struct UserUpdate {
    id: i32,
    name: String,
    email: String,
    active: bool,
}

// Sorgulamak için model
#[derive(Queryable)]
#[table("users")]
#[select("id, name, email, active")] 
#[where_clause("id = $")]
struct UserById {
    id: i32,
    name: String,
    email: String,
    active: bool,
}

// Silmek için model
#[derive(Deletable)]
#[table("users")]
#[where_clause("id = $")]
struct UserDelete {
    id: i32,
}
```

### CRUD İşlemleri

#### Veri Ekleme

```rust
use parsql_deadpool_postgres::insert;

let user = UserInsert {
    name: "Ahmet Yılmaz".to_string(),
    email: "ahmet@example.com".to_string(),
    active: true,
};

let result = insert(&pool, user).await?;
println!("Eklenen kayıt sayısı: {}", result);
```

#### Veri Güncelleme

```rust
use parsql_deadpool_postgres::update;

let user = UserUpdate {
    id: 1,
    name: "Ahmet Yılmaz (Güncellendi)".to_string(),
    email: "ahmet.updated@example.com".to_string(),
    active: true,
};

let success = update(&pool, user).await?;
println!("Güncelleme başarılı: {}", success);
```

#### Veri Sorgulama

```rust
use parsql_deadpool_postgres::{get, get_all};

// Tek bir kayıt getirme
let query = UserById { id: 1, ..Default::default() };
let user = get(&pool, &query).await?;

// Birden fazla kayıt getirme
let query = UsersByActive { active: true, ..Default::default() };
let active_users = get_all(&pool, &query).await?;
```

#### Veri Silme

```rust
use parsql_deadpool_postgres::delete;

let user_delete = UserDelete { id: 1 };
let deleted_count = delete(&pool, user_delete).await?;
println!("Silinen kayıt sayısı: {}", deleted_count);
```

### Özel Satır Dönüşümleri

Sorgu sonuçlarını farklı bir yapıya dönüştürmek için:

```rust
use parsql_deadpool_postgres::select_all;
use tokio_postgres::Row;

// Özet veri modeli
struct UserSummary {
    id: i32,
    full_name: String,
}

// Özel dönüşüm fonksiyonu ile sorgulama
let query = UsersByActive { active: true, ..Default::default() };

let summaries = select_all(&pool, query, |row: &Row| UserSummary {
    id: row.get("id"),
    full_name: row.get("name"),
}).await?;
```

### Transaction İşlemleri

```rust
// Transaction ile işlemler yapma
let client = pool.get().await?;
let tx = client.transaction().await?;

// İşlemler...
let user = UserInsert { ... };
let (tx, result) = parsql_deadpool_postgres::transactional::tx_insert(tx, user).await?;

// Transaction'ı commit etme
tx.commit().await?;
```

## Örnek Proje

Daha kapsamlı bir örnek için, proje içindeki `/examples/tokio-deadpool-postgres` dizinine bakabilirsiniz.

## Lisans

Bu proje MIT lisansı altında lisanslanmıştır.