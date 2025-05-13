use parsql::postgres::{
    get_all,
    macros::{FromRow, Queryable, SqlParams},
    traits::{FromRow, SqlParams, SqlQuery},
};
use postgres::{types::ToSql, Client, Error, Row};

/// İlk sayfa için sorgu yapısı
#[derive(Debug, Queryable, SqlParams, FromRow)]
#[table("users")]
#[where_clause("state >= $")]
#[order_by("id ASC")]
#[limit(5)] // Her sayfada 5 kayıt
#[offset(0)] // İlk sayfa (0. indeks)
pub struct PageOne {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub state: i16,
}

impl PageOne {
    pub fn new(min_state: i16) -> Self {
        Self {
            id: 0,
            name: String::new(),
            email: String::new(),
            state: min_state,
        }
    }
}

/// İkinci sayfa için sorgu yapısı
#[derive(Debug, Queryable, SqlParams, FromRow)]
#[table("users")]
#[where_clause("state >= $")]
#[order_by("id ASC")]
#[limit(5)] // Her sayfada 5 kayıt
#[offset(5)] // İkinci sayfa (5. kayıttan başla)
pub struct PageTwo {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub state: i16,
}

impl PageTwo {
    pub fn new(min_state: i16) -> Self {
        Self {
            id: 0,
            name: String::new(),
            email: String::new(),
            state: min_state,
        }
    }
}

/// Üçüncü sayfa için sorgu yapısı
#[derive(Debug, Queryable, SqlParams, FromRow)]
#[table("users")]
#[where_clause("state >= $")]
#[order_by("id ASC")]
#[limit(5)] // Her sayfada 5 kayıt
#[offset(10)] // Üçüncü sayfa (10. kayıttan başla)
pub struct PageThree {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub state: i16,
}

impl PageThree {
    pub fn new(min_state: i16) -> Self {
        Self {
            id: 0,
            name: String::new(),
            email: String::new(),
            state: min_state,
        }
    }
}

/// Dördüncü sayfa için sorgu yapısı
#[derive(Debug, Queryable, SqlParams, FromRow)]
#[table("users")]
#[where_clause("state >= $")]
#[order_by("id ASC")]
#[limit(5)] // Her sayfada 5 kayıt
#[offset(15)] // Dördüncü sayfa (15. kayıttan başla)
pub struct PageFour {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub state: i16,
}

impl PageFour {
    pub fn new(min_state: i16) -> Self {
        Self {
            id: 0,
            name: String::new(),
            email: String::new(),
            state: min_state,
        }
    }
}

/// Sadece aktif kullanıcıları (state=1) göstermek için sorgu yapısı
#[derive(Debug, Queryable, SqlParams, FromRow)]
#[table("users")]
#[where_clause("state = $")]
#[order_by("id ASC")]
#[limit(5)]
#[offset(0)]
pub struct ActiveUsers {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub state: i16,
}

impl ActiveUsers {
    pub fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
            email: String::new(),
            state: 1, // Aktif kullanıcılar için state=1
        }
    }
}

/// Queryable derive makrosu kullanan sayfalama örneğini çalıştıran fonksiyon
pub fn run_derive_pagination_examples(client: &mut Client) -> Result<(), Error> {
    println!("\n=== Derive Macro ile Sayfalama Örnekleri ===");

    // Kullanıcıların toplam sayısını kontrol et
    let count_result =
        client.query_one("SELECT COUNT(*) FROM users WHERE state >= $1", &[&0_i16])?;
    let total_users: i64 = count_result.get(0);
    println!("Toplam aktif kullanıcı sayısı: {}", total_users);

    // Sayfa 1 - İlk 5 kullanıcı
    let page1 = PageOne::new(0);
    let users_page1 = get_all(client, &page1)?;

    println!("\nSayfa 1 (0-4 arası kayıtlar) - Derive Macro ile:");
    for user in &users_page1 {
        println!(
            "ID: {}, İsim: {}, E-posta: {}, Durum: {}",
            user.id, user.name, user.email, user.state
        );
    }

    // Sayfa 2 - İkinci 5 kullanıcı
    let page2 = PageTwo::new(0);
    let users_page2 = get_all(client, &page2)?;

    println!("\nSayfa 2 (5-9 arası kayıtlar) - Derive Macro ile:");
    for user in &users_page2 {
        println!(
            "ID: {}, İsim: {}, E-posta: {}, Durum: {}",
            user.id, user.name, user.email, user.state
        );
    }

    // Sayfa 3 - Üçüncü 5 kullanıcı
    let page3 = PageThree::new(0);
    let users_page3 = get_all(client, &page3)?;

    println!("\nSayfa 3 (10-14 arası kayıtlar) - Derive Macro ile:");
    for user in &users_page3 {
        println!(
            "ID: {}, İsim: {}, E-posta: {}, Durum: {}",
            user.id, user.name, user.email, user.state
        );
    }

    // Sayfa 4 - Dördüncü 5 kullanıcı
    let page4 = PageFour::new(0);
    let users_page4 = get_all(client, &page4)?;

    println!("\nSayfa 4 (15-19 arası kayıtlar) - Derive Macro ile:");
    for user in &users_page4 {
        println!(
            "ID: {}, İsim: {}, E-posta: {}, Durum: {}",
            user.id, user.name, user.email, user.state
        );
    }

    // Sadece aktif kullanıcıları göster (state=1)
    println!("\nSadece aktif kullanıcılar (state=1) - Derive Macro ile:");
    let active_users_query = ActiveUsers::new();
    let active_users = get_all(client, &active_users_query)?;

    for user in &active_users {
        println!(
            "ID: {}, İsim: {}, E-posta: {}, Durum: {}",
            user.id, user.name, user.email, user.state
        );
    }

    Ok(())
}

/// Manuel sayfalama örneklerini çalıştıran ve sonuçları gösteren fonksiyon
pub fn run_pagination_examples(client: &mut Client) -> Result<(), Error> {
    println!("\n=== Sayfalama Örnekleri ===");

    // Kullanıcıların toplam sayısını kontrol et
    let count_result =
        client.query_one("SELECT COUNT(*) FROM users WHERE state >= $1", &[&0_i16])?;
    let total_users: i64 = count_result.get(0);
    println!("Toplam aktif kullanıcı sayısı: {}", total_users);

    // Sayfalama sorguları manuel olarak çalıştırılıyor
    let min_state: i16 = 0; // Minimum durum

    // Sayfa 1 - İlk 5 kullanıcı
    let users_page1 = client.query(
        "SELECT id, name, email, state FROM users WHERE state >= $1 ORDER BY id ASC LIMIT 5 OFFSET 0", 
        &[&min_state]
    )?;

    println!("\nSayfa 1 (0-4 arası kayıtlar):");
    for row in &users_page1 {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        let email: String = row.get("email");
        let state: i16 = row.get("state");
        println!(
            "ID: {}, İsim: {}, E-posta: {}, Durum: {}",
            id, name, email, state
        );
    }

    // Sayfa 2 - İkinci 5 kullanıcı
    let users_page2 = client.query(
        "SELECT id, name, email, state FROM users WHERE state >= $1 ORDER BY id ASC LIMIT 5 OFFSET 5", 
        &[&min_state]
    )?;

    println!("\nSayfa 2 (5-9 arası kayıtlar):");
    for row in &users_page2 {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        let email: String = row.get("email");
        let state: i16 = row.get("state");
        println!(
            "ID: {}, İsim: {}, E-posta: {}, Durum: {}",
            id, name, email, state
        );
    }

    // Sayfa 3 - Üçüncü 5 kullanıcı
    let users_page3 = client.query(
        "SELECT id, name, email, state FROM users WHERE state >= $1 ORDER BY id ASC LIMIT 5 OFFSET 10", 
        &[&min_state]
    )?;

    println!("\nSayfa 3 (10-14 arası kayıtlar):");
    for row in &users_page3 {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        let email: String = row.get("email");
        let state: i16 = row.get("state");
        println!(
            "ID: {}, İsim: {}, E-posta: {}, Durum: {}",
            id, name, email, state
        );
    }

    // Sayfa 4 - Dördüncü 5 kullanıcı
    let users_page4 = client.query(
        "SELECT id, name, email, state FROM users WHERE state >= $1 ORDER BY id ASC LIMIT 5 OFFSET 15", 
        &[&min_state]
    )?;

    println!("\nSayfa 4 (15-19 arası kayıtlar):");
    for row in &users_page4 {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        let email: String = row.get("email");
        let state: i16 = row.get("state");
        println!(
            "ID: {}, İsim: {}, E-posta: {}, Durum: {}",
            id, name, email, state
        );
    }

    // Sadece aktif kullanıcıları göster (state=1)
    println!("\nSadece aktif kullanıcılar (state=1):");
    let active_state: i16 = 1;
    let active_users = client.query(
        "SELECT id, name, email, state FROM users WHERE state = $1 ORDER BY id ASC LIMIT 5",
        &[&active_state],
    )?;

    for row in &active_users {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        let email: String = row.get("email");
        let state: i16 = row.get("state");
        println!(
            "ID: {}, İsim: {}, E-posta: {}, Durum: {}",
            id, name, email, state
        );
    }

    Ok(())
}
