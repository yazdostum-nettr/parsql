# parsql
Deneyimsel SQL yardımcısı

## Ne İşe Yarar?

Parsql, SQL sorgularınızı doğrudan Rust struct'ları üzerinden yönetmenize olanak tanıyan bir kütüphanedir. Temel amacı, veritabanı işlemlerini daha güvenli ve daha az kod ile gerçekleştirmenizi sağlamaktır. Bu kütüphane ile:

- Struct tanımları üzerinden otomatik SQL sorguları oluşturabilirsiniz
- Veritabanı parametrelerini güvenli bir şekilde yönetebilirsiniz
- Generic CRUD işlemlerini (ekleme, okuma, güncelleme, silme) kolayca yapabilirsiniz
- Dinamik SQL oluşturabilir ve karmaşık sorgular çalıştırabilirsiniz

Parsql standart bir ORM değildir. Daha çok, SQL yazımını ve kullanımını basitleştirmeye odaklanır.

## Desteklenen Veritabanları

Parsql aşağıdaki veritabanı sistemlerini desteklemektedir:

- **SQLite** (senkron): `parsql-sqlite` paketi
- **PostgreSQL** (senkron): `parsql-postgres` paketi
- **Tokio PostgreSQL** (asenkron): `parsql-tokio-postgres` paketi

## Küfe'nin Yüklenmesi

Uygulamanıza küfeyi yüklerken hangi veritabanı ile çalışacağınızı 'feature' olarak belirtmeniz gerekiyor. Cargo.toml dosyanıza paketi şu şekilde ekleyebilirsiniz:

### SQLite için
```rust
parsql = { version = "0.3.1", features = ["sqlite"] }
```

### PostgreSQL için
```rust
parsql = { version = "0.3.1", features = ["postgres"] }
```

### Tokio PostgreSQL için
```rust
parsql = { version = "0.3.1", features = ["tokio-postgres"] }
```

### Deadpool PostgreSQL bağlantı havuzu için
```rust
parsql = { version = "0.3.1", features = ["deadpool-postgres"] }
```

## Temel Özellikler

### Procedural Makrolar
Parsql, veritabanı işlemlerini kolaylaştırmak için çeşitli procedural makrolar sunar:

- `#[derive(Queryable)]` - Okuma (select) işlemleri için
- `#[derive(Insertable)]` - Ekleme işlemleri için
- `#[derive(Updateable)]` - Güncelleme işlemleri için
- `#[derive(FromRow)]` - Veritabanı sonuçlarını nesnelere dönüştürmek için

### Güvenlik Özellikleri

#### SQL Injection Koruması
Parsql, SQL injection saldırılarına karşı güvenli bir şekilde tasarlanmıştır:

- Parametreli sorgular otomatik olarak kullanılır, asla direk string birleştirme yapılmaz
- Tüm kullanıcı girdileri güvenli bir şekilde parametrize edilir
- Makrolar, SQL parametrelerini doğru bir şekilde işler ve güvenli bir format sağlar
- Her veritabanı adaptörü için uygun parametre işaretleyiciler (`$1`, `?`, vb.) otomatik olarak uygulanır
- SQL yazarken elle string birleştirme gereksinimi ortadan kaldırılmıştır

```rust
// Güvenli parametre kullanımı örneği
#[derive(Queryable, FromRow, SqlParams)]
#[table("users")]
#[where_clause("username = $ AND status = $")]
struct UserQuery {
    username: String,
    status: String,
}

// Parametreler güvenli bir şekilde yerleştirilir, 
// SQL injection riski olmaz
let query = UserQuery {
    username: user_input,
    status: "active",
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

### SQL İzleme
Geliştirme sırasında oluşturulan SQL sorgularını izlemek için:

```sh
PARSQL_TRACE=1 cargo run
```

## Basit Kullanım Örneği

```rust
// Bir kayıt almak için
#[derive(Queryable, FromRow, SqlParams, Debug)]
#[table("users")]
#[where_clause("id = $")]
pub struct GetUser {
    pub id: i64,
    pub name: String,
    pub email: String,
}

// SQLite için kullanım
let get_user = GetUser::new(1);
let user = get(&conn, get_user);

// Tokio-Postgres için kullanım
let get_user = GetUser::new(1);
let user = get(&client, get_user).await;
```

## Detaylı Dökümantasyon

Her veritabanı adaptörü için daha detaylı bilgi ve örnekler, ilgili alt paketlerin README dosyalarında bulunmaktadır:

- [SQLite Dökümantasyonu](./parsql-sqlite/README.md)
- [PostgreSQL Dökümantasyonu](./parsql-postgres/README.md)
- [Tokio PostgreSQL Dökümantasyonu](./parsql-tokio-postgres/README.md)

GitHub'daki [örnekler klasöründe](./examples) her veritabanı tipi için kapsamlı örnek projeler bulabilirsiniz.

## 0.3.0 Sürümündeki Değişiklikler

- `join`, `group_by`, `order_by` ve `having` öznitelikleri eklendi
- `PARSQL_TRACE` çevre değişkeni desteği eklendi
- Öznitelik isimleri güncellendi (`table_name`→`table`, `update_clause`→`update`, `select_clause`→`select`)
- `SqlQuery` trait'i eklendi ve trait yapısı sadeleştirildi
- Temel trait'ler `parsql-core` crate'inde toplandı
