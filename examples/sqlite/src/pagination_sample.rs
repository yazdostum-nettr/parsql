use parsql::macros::{Queryable, SqlParams, FromRow};
use parsql::sqlite::{get_all, SqlQuery, SqlParams, FromRow};
use rusqlite::{Connection, Result, Error, Row, types::ToSql, params};

/// İlk sayfa için sorgu yapısı
#[derive(Debug, Queryable, SqlParams, FromRow)]
#[table("users")]
#[where_clause("state >= $")]
#[order_by("id ASC")]
#[limit(5)]       // Her sayfada 5 kayıt
#[offset(0)]      // İlk sayfa (0. indeks)
pub struct PageOne {
    pub id: i64,
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
#[limit(5)]       // Her sayfada 5 kayıt
#[offset(5)]      // İkinci sayfa (5. kayıttan başla)
pub struct PageTwo {
    pub id: i64,
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
#[limit(5)]       // Her sayfada 5 kayıt
#[offset(10)]     // Üçüncü sayfa (10. kayıttan başla)
pub struct PageThree {
    pub id: i64,
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

/// Sadece aktif kullanıcıları (state=1) göstermek için sorgu yapısı
#[derive(Debug, Queryable, SqlParams, FromRow)]
#[table("users")]
#[where_clause("state = $")]
#[order_by("id ASC")]
#[limit(5)]
#[offset(0)]
pub struct ActiveUsers {
    pub id: i64,
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
pub fn run_derive_pagination_examples(conn: &Connection) -> Result<()> {
    println!("\n=== Derive Macro ile Sayfalama Örnekleri (SQLite) ===");

    // Kullanıcıların toplam sayısını kontrol et
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE state >= ?")?;
    let total_users: i64 = stmt.query_row(params![0_i16], |row| row.get(0))?;
    println!("Toplam kullanıcı sayısı: {}", total_users);

    // Sayfa 1 - İlk 5 kullanıcı
    let page1 = PageOne::new(0);
    let users_page1 = get_all(conn, &page1)?;
    
    println!("\nSayfa 1 (0-4 arası kayıtlar) - Derive Macro ile:");
    for user in &users_page1 {
        println!("ID: {}, İsim: {}, E-posta: {}, Durum: {}", 
                user.id, user.name, user.email, user.state);
    }
    
    // Sayfa 2 - İkinci 5 kullanıcı
    let page2 = PageTwo::new(0);
    let users_page2 = get_all(conn, &page2)?;
    
    println!("\nSayfa 2 (5-9 arası kayıtlar) - Derive Macro ile:");
    for user in &users_page2 {
        println!("ID: {}, İsim: {}, E-posta: {}, Durum: {}", 
                user.id, user.name, user.email, user.state);
    }
    
    // Sayfa 3 - Üçüncü 5 kullanıcı
    let page3 = PageThree::new(0);
    let users_page3 = get_all(conn, &page3)?;
    
    println!("\nSayfa 3 (10-14 arası kayıtlar) - Derive Macro ile:");
    for user in &users_page3 {
        println!("ID: {}, İsim: {}, E-posta: {}, Durum: {}", 
                user.id, user.name, user.email, user.state);
    }
    
    // Sadece aktif kullanıcıları göster (state=1)
    println!("\nSadece aktif kullanıcılar (state=1) - Derive Macro ile:");
    let active_users_query = ActiveUsers::new();
    let active_users = get_all(conn, &active_users_query)?;
    
    for user in &active_users {
        println!("ID: {}, İsim: {}, E-posta: {}, Durum: {}", 
                user.id, user.name, user.email, user.state);
    }
    
    Ok(())
}

/// Manuel sayfalama örneklerini çalıştıran ve sonuçları gösteren fonksiyon
pub fn run_pagination_examples(conn: &Connection) -> Result<()> {
    println!("\n=== Manuel Sayfalama Örnekleri (SQLite) ===");

    // Kullanıcıların toplam sayısını kontrol et
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE state >= ?")?;
    let total_users: i64 = stmt.query_row(params![0_i16], |row| row.get(0))?;
    println!("Toplam kullanıcı sayısı: {}", total_users);

    // Sayfalama sorguları manuel olarak çalıştırılıyor
    let min_state: i16 = 0; // Minimum durum

    // Sayfa 1 - İlk 5 kullanıcı
    let mut stmt = conn.prepare(
        "SELECT id, name, email, state FROM users WHERE state >= ? ORDER BY id ASC LIMIT 5 OFFSET 0"
    )?;
    
    let users_page1 = stmt.query_map(params![&min_state], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i16>(3)?,
        ))
    })?;
    
    println!("\nSayfa 1 (0-4 arası kayıtlar):");
    for user in users_page1 {
        if let Ok((id, name, email, state)) = user {
            println!("ID: {}, İsim: {}, E-posta: {}, Durum: {}", id, name, email, state);
        }
    }
    
    // Sayfa 2 - İkinci 5 kullanıcı
    let mut stmt = conn.prepare(
        "SELECT id, name, email, state FROM users WHERE state >= ? ORDER BY id ASC LIMIT 5 OFFSET 5"
    )?;
    
    let users_page2 = stmt.query_map(params![&min_state], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i16>(3)?,
        ))
    })?;
    
    println!("\nSayfa 2 (5-9 arası kayıtlar):");
    for user in users_page2 {
        if let Ok((id, name, email, state)) = user {
            println!("ID: {}, İsim: {}, E-posta: {}, Durum: {}", id, name, email, state);
        }
    }
    
    // Sayfa 3 - Üçüncü 5 kullanıcı
    let mut stmt = conn.prepare(
        "SELECT id, name, email, state FROM users WHERE state >= ? ORDER BY id ASC LIMIT 5 OFFSET 10"
    )?;
    
    let users_page3 = stmt.query_map(params![&min_state], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i16>(3)?,
        ))
    })?;
    
    println!("\nSayfa 3 (10-14 arası kayıtlar):");
    for user in users_page3 {
        if let Ok((id, name, email, state)) = user {
            println!("ID: {}, İsim: {}, E-posta: {}, Durum: {}", id, name, email, state);
        }
    }
    
    // Sadece aktif kullanıcıları göster (state=1)
    let active_state: i16 = 1;
    let mut stmt = conn.prepare(
        "SELECT id, name, email, state FROM users WHERE state = ? ORDER BY id ASC LIMIT 5"
    )?;
    
    let active_users = stmt.query_map(params![&active_state], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, i16>(3)?,
        ))
    })?;
    
    println!("\nSadece aktif kullanıcılar (state=1):");
    for user in active_users {
        if let Ok((id, name, email, state)) = user {
            println!("ID: {}, İsim: {}, E-posta: {}, Durum: {}", id, name, email, state);
        }
    }
    
    Ok(())
} 