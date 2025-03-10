# parsql
Deneyimsel SQL yardımcısı

## Ne İşe Yarar?

Parsql, SQL sorgularınızı doğrudan Rust struct'ları üzerinden yönetmenize olanak tanıyan bir kütüphanedir. Temel amacı, veritabanı işlemlerini daha güvenli ve daha az kod ile gerçekleştirmenizi sağlamaktır. Bu kütüphane ile:

- Struct tanımları üzerinden otomatik SQL sorguları oluşturabilirsiniz
- Veritabanı parametrelerini güvenli bir şekilde yönetebilirsiniz
- Generic CRUD işlemlerini (ekleme, okuma, güncelleme, silme) kolayca yapabilirsiniz
- Dinamik SQL oluşturabilir ve karmaşık sorgular çalıştırabilirsiniz
- Asenkron veritabanı işlemlerini kolayca gerçekleştirebilirsiniz
- SQL injection saldırılarına karşı otomatik koruma sağlayabilirsiniz

Parsql standart bir ORM değildir. Daha çok, SQL yazımını ve kullanımını basitleştirmeye odaklanır.

## Desteklenen Veritabanları

Parsql aşağıdaki veritabanı sistemlerini desteklemektedir:

- **SQLite** (senkron): `parsql-sqlite` paketi
- **PostgreSQL** (senkron): `parsql-postgres` paketi
- **Tokio PostgreSQL** (asenkron): `parsql-tokio-postgres` paketi
- **Deadpool PostgreSQL** (asenkron bağlantı havuzu): `parsql-deadpool-postgres` paketi

## Küfe'nin Yüklenmesi

Uygulamanıza küfeyi yüklerken hangi veritabanı ile çalışacağınızı 'feature' olarak belirtmeniz gerekiyor. Cargo.toml dosyanıza paketi şu şekilde ekleyebilirsiniz:

### SQLite için
```toml
parsql = { version = "0.3.2", features = ["sqlite"] }
```

### PostgreSQL için
```toml
parsql = { version = "0.3.2", features = ["postgres"] }
```

### Tokio PostgreSQL için
```toml
parsql = { version = "0.3.2", features = ["tokio-postgres"] }
```

### Deadpool PostgreSQL bağlantı havuzu için
```toml
parsql = { version = "0.3.2", features = ["deadpool-postgres"] }
```

## Temel Özellikler

### Procedural Makrolar
Parsql, veritabanı işlemlerini kolaylaştırmak için çeşitli procedural makrolar sunar:

- `#[derive(Queryable)]` - Okuma (select) işlemleri için
- `#[derive(Insertable)]` - Ekleme işlemleri için
- `#[derive(Updateable)]` - Güncelleme işlemleri için
- `#[derive(Deletable)]` - Silme işlemleri için
- `#[derive(FromRow)]` - Veritabanı sonuçlarını nesnelere dönüştürmek için
- `#[derive(SqlParams)]` - SQL parametrelerini yapılandırmak için
- `#[derive(UpdateParams)]` - Güncelleme parametrelerini yapılandırmak için

### Güvenlik Özellikleri

#### SQL Injection Koruması
Parsql, SQL injection saldırılarına karşı güvenli bir şekilde tasarlanmıştır:

- Parametreli sorgular otomatik olarak kullanılır, asla direk string birleştirme yapılmaz
- Tüm kullanıcı girdileri güvenli bir şekilde parametrize edilir
- Makrolar, SQL parametrelerini doğru bir şekilde işler ve güvenli bir format sağlar
- Her veritabanı adaptörü için uygun parametre işaretleyiciler (`$1`, `?`, vb.) otomatik olarak uygulanır
- SQL yazarken elle string birleştirme gereksinimi ortadan kaldırılmıştır
- Asenkron bağlamlarda bile güvenlik önlemleri tam olarak korunur

```rust
// Güvenli parametre kullanımı örneği
#[derive(Queryable, FromRow, SqlParams)]
#[table("users")]
#[where_clause("username = $ AND status = $")]
struct UserQuery {
    username: String,
    status: i32,
}

// Parametreler güvenli bir şekilde yerleştirilir, 
// SQL injection riski olmaz
let query = UserQuery {
    username: user_input,
    status: 1,
};
```

### Öznitelikler
Sorgularınızı özelleştirmek için çeşitli öznitelikler kullanabilirsiniz:

- `#[table("tablo_adi")]` - Tablo adını belirtmek için
- `#[where_clause("id = $")]` - WHERE koşulunu belirtmek için
- `#[select("alan1, alan2")]` - SELECT ifadesini özelleştirmek için
- `#[update("alan1, alan2")]` - UPDATE ifadesini özelleştirmek için
- `#[join("LEFT JOIN tablo2 ON tablo1.id = tablo2.fk_id")]` - JOIN ifadeleri için
- `#[group_by("alan1")]` - GROUP BY ifadesi için
- `#[order_by("alan1 DESC")]` - ORDER BY ifadesi için
- `#[having("COUNT(*) > 5")]` - HAVING ifadesi için
- `#[limit(10)]` - LIMIT ifadesi için
- `#[offset(5)]` - OFFSET ifadesi için
- `#[returning("id")]` - INSERT/UPDATE işlemlerinden dönen değerleri belirtmek için

### SQL İzleme
Geliştirme sırasında oluşturulan SQL sorgularını izlemek için:

```sh
PARSQL_TRACE=1 cargo run
```

Bu, çalıştırılan tüm SQL sorgularını konsola yazdıracaktır.

## Basit Kullanım Örnekleri

### SQLite ile Kullanım

```rust
use parsql::{
    sqlite::{get, insert},
    macros::{Queryable, Insertable, FromRow, SqlParams},
};
use rusqlite::Connection;

// Bir kayıt almak için
#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[where_clause("id = $")]
pub struct GetUser {
    pub id: i64,
    pub name: String,
    pub email: String,
}

impl GetUser {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            name: Default::default(),
            email: Default::default(),
        }
    }
}

// Yeni kayıt eklemek için
#[derive(Insertable, SqlParams)]
#[table("users")]
pub struct InsertUser {
    pub name: String,
    pub email: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("test.db")?;
    
    let insert_user = InsertUser {
        name: "Ali".to_string(),
        email: "ali@example.com".to_string(),
    };
    
    let id = insert(&conn, insert_user)?;
    println!("Eklenen kayıt ID: {}", id);
    
    let get_user = GetUser::new(id);
    let user = get(&conn, get_user)?;
    println!("Kullanıcı: {:?}", user);
    
    Ok(())
}
```

### Tokio-Postgres ile Asenkron Kullanım

```rust
use parsql::{
    tokio_postgres::{get, insert},
    macros::{Queryable, Insertable, FromRow, SqlParams},
};
use tokio_postgres::{NoTls, Error};

#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[where_clause("id = $")]
pub struct GetUser {
    pub id: i64,
    pub name: String,
    pub email: String,
}

impl GetUser {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            name: Default::default(),
            email: Default::default(),
        }
    }
}

#[derive(Insertable, SqlParams)]
#[table("users")]
pub struct InsertUser {
    pub name: String,
    pub email: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres dbname=test",
        NoTls,
    ).await?;
    
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Bağlantı hatası: {}", e);
        }
    });
    
    let insert_user = InsertUser {
        name: "Ali".to_string(),
        email: "ali@example.com".to_string(),
    };
    
    let id = insert(&client, insert_user).await?;
    println!("Eklenen kayıt ID: {}", id);
    
    let get_user = GetUser::new(id);
    let user = get(&client, get_user).await?;
    println!("Kullanıcı: {:?}", user);
    
    Ok(())
}
```

## Performans İpuçları

- Aynı SQL yapısına sahip sorguları tekrar kullanarak sorgu planı ön belleğinden yararlanın
- Yoğun veritabanı uygulamaları için bağlantı havuzları kullanın
- Büyük veri kümeleri için `get_all` yerine sayfalama (limit ve offset) kullanın
- Filtreleri veritabanı seviyesinde uygulayın, uygulamanızda değil

## Detaylı Dökümantasyon

Her veritabanı adaptörü için daha detaylı bilgi ve örnekler, ilgili alt paketlerin README dosyalarında bulunmaktadır:

- [SQLite Dökümantasyonu](./parsql-sqlite/README.md)
- [PostgreSQL Dökümantasyonu](./parsql-postgres/README.md)
- [Tokio PostgreSQL Dökümantasyonu](./parsql-tokio-postgres/README.md)
- [Deadpool PostgreSQL Dökümantasyonu](./parsql-deadpool-postgres/README.md)

GitHub'daki [örnekler klasöründe](./examples) her veritabanı tipi için kapsamlı örnek projeler bulabilirsiniz.

## 0.3.0 Sürümündeki Değişiklikler

- `join`, `group_by`, `order_by` ve `having` öznitelikleri eklendi
- `PARSQL_TRACE` çevre değişkeni desteği eklendi
- Öznitelik isimleri güncellendi (`table_name`→`table`, `update_clause`→`update`, `select_clause`→`select`)
- `SqlQuery` trait'i eklendi ve trait yapısı sadeleştirildi
- `parsql-tokio-postgres` paketinde bir özellik olarak mevcut olan `deadpool-postgres`, `parsql-deadpool-postgres` paketi olarak yeniden yapılandırıldı

## Lisanslama

Bu kütüphane MIT veya Apache-2.0 lisansı altında lisanslanmıştır.
