use parsql::tokio_postgres::{
    get_all,
    macros::FromRow,
    macros::{Queryable, SqlParams},
    traits::{FromRow, SqlParams, SqlQuery},
};
use tokio_postgres::{types::ToSql, Client, Error, Row};

/// Limit ve offset özelliklerini gösteren örnek yapı
/// Bu yapı users tablosundaki kayıtları sayfalayarak alabilecek.
#[derive(Debug, Queryable, SqlParams, FromRow)]
#[table("users")]
#[select("id, name, email, state")]
#[where_clause("state = $1")]
#[order_by("id ASC")]
#[limit(5)] // Her sayfa için 5 kayıt
#[offset(0)] // İlk sayfa (0. indeks)
pub struct GetUsers {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

impl GetUsers {
    pub fn new(state: i16) -> Self {
        Self {
            id: 0,
            name: String::new(),
            email: String::new(),
            state,
        }
    }
}

/// İkinci sayfa için kullanılabilecek yapı
#[derive(Debug, Queryable, SqlParams, FromRow)]
#[table("users")]
#[select("id, name, email, state")]
#[where_clause("state = $1")]
#[order_by("id ASC")]
#[limit(5)] // Her sayfa için 5 kayıt
#[offset(5)] // İkinci sayfa (5. indeks başlangıcı)
pub struct GetUsersPage2 {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

impl GetUsersPage2 {
    pub fn new(state: i16) -> Self {
        Self {
            id: 0,
            name: String::new(),
            email: String::new(),
            state,
        }
    }
}

/// Sayfalama işlemi için dinamik bir yapı
/// Offset değeri çalışma zamanında belirlenemiyor, bu nedenle
/// farklı sayfalar için farklı yapılar oluşturulmalı.
#[derive(Debug, Queryable, SqlParams, FromRow)]
#[table("users")]
#[select("id, name, email, state")]
#[where_clause("state = $1")]
#[order_by("id ASC")]
#[limit(10)] // Her sayfa için 10 kayıt
pub struct GetUsersPage {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub state: i16,
}

impl GetUsersPage {
    pub fn new(state: i16) -> Self {
        Self {
            id: 0,
            name: String::new(),
            email: String::new(),
            state,
        }
    }
}

/// Limit ve offset özelliklerini kullanan örnek fonksiyon
pub async fn list_users_with_pagination(client: &Client) -> Result<(), Error> {
    println!("=== Limit ve Offset örnekleri ===");

    // İlk sayfa (ilk 5 kayıt)
    let active_users = GetUsers::new(1);
    let users = get_all(client, active_users).await?;

    println!("İlk sayfa (5 kayıt):");
    for user in users {
        println!("Kullanıcı: {} ({})", user.name, user.email);
    }

    // İkinci sayfa (ikinci 5 kayıt)
    let active_users_page2 = GetUsersPage2::new(1);
    let users = get_all(client, active_users_page2).await?;

    println!("\nİkinci sayfa (5 kayıt):");
    for user in users {
        println!("Kullanıcı: {} ({})", user.name, user.email);
    }

    // Not: Gerçek bir uygulamada, sayfa numarasına göre farklı
    // yapılar oluşturmak yerine, sorgu parametreleriyle LIMIT ve OFFSET
    // değerlerini dinamik olarak belirlemek daha doğru olacaktır.

    Ok(())
}
