# Parsql Tokio-Postgres Örnekleri

Bu dizin, Parsql'in Tokio-Postgres entegrasyonunu gösteren örnek kodları içerir. 

Bu örnekler, parsql-tokio-postgres kütüphanesinde yeni eklenen CrudOps trait'inin, derive makroları ile birlikte kullanımını gösterir. CrudOps trait'i, Client nesnesine extension method olarak çalışır ve SQL sorgularını otomatik oluşturur.

## Örnekleri Çalıştırma

Örnekleri çalıştırmadan önce, PostgreSQL veritabanınızın çalıştığından ve bağlantı bilgilerinin kod içinde doğru olduğundan emin olun.

```bash
# Varsayılan örneği (makro örneği) çalıştırma
cargo run --bin ex_parsql_tokio_pg

# Makro örneğini açıkça belirterek çalıştırma
cargo run --bin ex_parsql_tokio_pg -- macro

# Eski CrudOps örneğini çalıştırma
cargo run --bin ex_parsql_tokio_pg -- crud
```

## Derive Makroları ve CrudOps Trait Kullanımı

Parsql, SQL sorgularını otomatik oluşturmak için derive makrolarını kullanır. Bu makrolar, yapıları SQL sorgularına dönüştürür ve CrudOps trait'i ile birlikte çalışır.

### Derive Makroları

Parsql'in sunduğu makrolar:

- `Queryable`: SELECT sorguları için
- `Insertable`: INSERT sorguları için
- `Updateable`: UPDATE sorguları için
- `Deletable`: DELETE sorguları için
- `SqlParams`: SQL parametreleri için
- `UpdateParams`: UPDATE parametreleri için
- `FromRow`: Veritabanı satırlarını yapılara dönüştürmek için

### Örnek Kullanım

Veri modelleri için makroları şu şekilde kullanabilirsiniz:

```rust
// Kullanıcı tablosundan veri almak için model
#[derive(Debug, Queryable, FromRow, SqlParams)]
#[table("users")]
#[where_clause("id = $")]
pub struct GetUser {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

// Yeni kullanıcı eklemek için model
#[derive(Insertable, SqlParams)]
#[table("users")]
pub struct InsertUser {
    pub name: String,
    pub email: String,
    pub state: i16,
}

// Kullanıcı güncellemek için model
#[derive(Updateable, UpdateParams)]
#[table("users")]
#[update("name, email")]
#[where_clause("id = $")]
pub struct UpdateUser {
    pub id: i64,
    pub name: String,
    pub email: String,
}

// Kullanıcı silmek için model
#[derive(Deletable, SqlParams)]
#[table("users")]
#[where_clause("id = $")]
pub struct DeleteUser {
    pub id: i64,
}
```

### CrudOps Trait Kullanımı

CrudOps trait'i, Client nesnesine uzantı metotları ekler. Bu metotlar, derive makrolarıyla oluşturulan yapılarla birlikte çalışır:

```rust
// Kullanıcı getirme
let user = client.get(GetUser::new(1)).await?;

// Kullanıcı ekleme
let id = client.insert(InsertUser { 
    name: "Ali".to_string(), 
    email: "ali@example.com".to_string(), 
    state: 1 
}).await?;

// Kullanıcı güncelleme
let updated = client.update(UpdateUser { 
    id: 1, 
    name: "Ali (Güncel)".to_string(), 
    email: "ali.updated@example.com".to_string() 
}).await?;

// Kullanıcı silme
let deleted = client.delete(DeleteUser { id: 3 }).await?;
```

### Makrolar Hakkında Detaylar

#### Attribute Makroları

Parsql'in derive makrolarının yanında, model yapılarını şekillendirmek için kullanabileceğiniz attribute makroları da vardır:

- `#[table("tablo_adı")]`: SQL sorgusunun çalışacağı tablo adını belirtir
- `#[where_clause("koşul")]`: WHERE koşulunu belirtir, $ işareti parametre yerini gösterir
- `#[update("alan1, alan2")]`: UPDATE sorgusunda güncellenecek alanları belirtir
- `#[select("özel sorgu")]`: Özel bir SELECT sorgusu tanımlar
- `#[order_by("sıralama")]`: ORDER BY ifadesini belirtir
- `#[group_by("gruplama")]`: GROUP BY ifadesini belirtir
- `#[having("koşul")]`: HAVING koşulunu belirtir

## SQL Enjeksiyon Koruması

Parsql, SQL enjeksiyon saldırılarına karşı koruma sağlar. Tüm parametreler, uygun binding ile gönderilir ve sorgu metni ile ayrılır. Bu, SQL enjeksiyon saldırılarını engeller.

## Performans

Derive makroları, derleme zamanında çalışır ve runtime performansını etkilemez. CrudOps trait'i, veritabanı işlemlerini optimize eder ve hızlı çalışmasını sağlar. 