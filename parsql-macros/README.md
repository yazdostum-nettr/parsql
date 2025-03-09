# parsql-macros

Parsql için procedural makroları barındıran küfedir. Bu paket, SQL sorgu oluşturma ve parametre işleme için derive makrolarını içerir.

## Özellikler

- Otomatik SQL sorgu oluşturma
- Güvenli parametre yönetimi
- Birden fazla veritabanı sistemi için destek (PostgreSQL, SQLite)
- Tip güvenliği olan veritabanı işlemleri
- SQL Injection saldırılarına karşı otomatik koruma

## Makrolar

- `Updateable`: UPDATE sorgularını oluşturur
- `Insertable`: INSERT sorgularını oluşturur
- `Queryable`: SELECT sorgularını oluşturur
- `Deletable`: DELETE sorgularını oluşturur
- `SqlParams`: Parametre işleme kodunu oluşturur
- `UpdateParams`: UPDATE işlemleri için parametre işleme kodunu oluşturur
- `FromRow`: Veritabanı satırlarını Rust yapılarına dönüştürmek için kod oluşturur

## Kurulum

Cargo.toml dosyanıza şu şekilde ekleyin:

```toml
[dependencies]
parsql-macros = "0.3.1"
```

ve özellik seçimini yapın:

```toml
# SQLite için
parsql-macros = { version = "0.3.1", features = ["sqlite"] }

# PostgreSQL için
parsql-macros = { version = "0.3.1", features = ["postgres"] }

# Tokio PostgreSQL için
parsql-macros = { version = "0.3.1", features = ["tokio-postgres"] }

# Deadpool PostgreSQL için
parsql-macros = { version = "0.3.1", features = ["deadpool-postgres"] }
```

## Güvenlik Özellikleri

### SQL Injection Koruması

parsql-macros, SQL Injection saldırılarına karşı güvenli bir şekilde tasarlanmıştır:

- Tüm makrolar, kullanıcı verilerini doğrudan SQL sorgularına dahil etmek yerine, parametreleri kullanır
- WHERE koşulları ve diğer SQL bileşenleri güvenli bir şekilde parametrize edilir
- Her veritabanı adaptörü için uygun parametre işaretleyicileri (`$1`, `?`, vb.) otomatik olarak oluşturulur
- Parametre değerlendirme sırası korunarak sorgu tutarlılığı sağlanır
- Özel karakter kaçışları ve SQL injection saldırıları otomatik olarak engellenir

## Kullanım Örnekleri

### `Queryable` Kullanımı

```rust
#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[where_clause("id = $")]
pub struct GetUser {
    pub id: i64,
    pub name: String,
    pub email: String,
}

// "SELECT id, name, email FROM users WHERE id = ?" sorgusu otomatik oluşturulur
// ve "id" parametresi güvenli bir şekilde yerleştirilir
```

### `Insertable` Kullanımı

```rust
#[derive(Insertable)]
#[table("users")]
pub struct InsertUser {
    pub name: String,
    pub email: String,
    pub status: i32,
}

// "INSERT INTO users (name, email, status) VALUES (?, ?, ?)" sorgusu otomatik oluşturulur
// ve tüm alanlar güvenli bir şekilde parametre olarak eklenir
```

### `Updateable` Kullanımı

```rust
#[derive(Updateable, UpdateParams)]
#[table("users")]
#[update("name, email")]
#[where_clause("id = $")]
pub struct UpdateUser {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub status: i32,
}

// "UPDATE users SET name = ?, email = ? WHERE id = ?" sorgusu otomatik oluşturulur
// ve değerler güvenli bir şekilde yerleştirilir
```

### `Deletable` Kullanımı

```rust
#[derive(Deletable, SqlParams)]
#[table("users")]
#[where_clause("id = $")]
pub struct DeleteUser {
    pub id: i64,
}

// "DELETE FROM users WHERE id = ?" sorgusu otomatik oluşturulur
// ve "id" parametresi güvenli bir şekilde yerleştirilir
```

## Öznitelikler

- `#[table("tablo_adi")]` - Sorgunun çalışacağı tablo adını belirtir
- `#[where_clause("koşul")]` - WHERE koşulunu tanımlar ($ işareti parametre yerini gösterir)
- `#[select("alan1, alan2")]` - SELECT sorgusu için hangi alanların seçileceğini belirtir
- `#[update("alan1, alan2")]` - UPDATE sorgusu için hangi alanların güncelleneceğini belirtir
- `#[join("LEFT JOIN tablo2 ON tablo1.id = tablo2.id")]` - JOIN ifadelerini belirtir
- `#[group_by("alan1")]` - GROUP BY ifadesini belirtir
- `#[order_by("alan1 DESC")]` - ORDER BY ifadesini belirtir
- `#[having("COUNT(*) > 5")]` - HAVING ifadesini belirtir

## Parametre İşaretleme

Her veritabanı için, uygun parametre işaretleme otomatik olarak yapılır:

- SQLite: `?` işareti kullanılır 
- PostgreSQL: `$1, $2, $3, ...` şeklinde numaralandırılmış parametreler kullanılır

## Lisans

[MIT license](../LICENSE)
