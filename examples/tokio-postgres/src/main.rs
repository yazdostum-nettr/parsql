mod crud_ops_sample;
mod macro_sample;
mod delete_sample;
mod get_sample;
mod insert_sample;
mod update_sample;
mod db_connection;

use std::env;
use dotenvy::dotenv;
use crate::crud_ops_sample::run_crud_ops_example;
use crate::macro_sample::run_macro_example;

#[tokio::main]
async fn main() {
    // .env dosyasını yükle
    if let Err(e) = dotenv() {
        eprintln!(".env dosyası yüklenemedi: {}, varsayılan değerler kullanılacak", e);
    }

    // Komut satırı argümanlarını parse et
    let args: Vec<String> = env::args().collect();
    
    // Veritabanı bağlantısını kurma
    let client = match db_connection::create_connection().await {
        Ok((client, connection)) => {
            // Bağlantıyı arka planda çalıştır
            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("Bağlantı hatası: {}", e);
                }
            });
            
            client
        },
        Err(e) => {
            eprintln!("Veritabanı bağlantısı kurulamadı: {}", e);
            return;
        }
    };

    // Tabloları oluştur ve örnek verileri yükle
    if let Err(e) = db_connection::setup_database(&client).await {
        eprintln!("Veritabanı hazırlama hatası: {}", e);
        return;
    }

    if let Err(e) = db_connection::seed_example_data_if_empty(&client).await {
        eprintln!("Örnek veri ekleme hatası: {}", e);
        return;
    }
    
    println!("Veritabanı bağlantısı kuruldu ve hazırlandı.");
    
    // Belirli bir örnek çalıştırılacak mı kontrol et
    if args.len() > 1 {
        match args[1].as_str() {
            "crud_ops" => {
                // Derive makroları ile CrudOps trait örneğini çalıştır
                if let Err(e) = run_crud_ops_example().await {
                    eprintln!("CrudOps örneği çalıştırılırken hata: {}", e);
                }
                return;
            },
            "macro" => {
                // Derive makroları kullanarak CrudOps trait örneğini çalıştır
                if let Err(e) = run_macro_example().await {
                    eprintln!("Makro örneği çalıştırılırken hata: {}", e);
                }
                return;
            },
            _ => {
                println!("Bilinmeyen örnek: {}", args[1]);
                println!("Kullanılabilir örnekler: crud_ops, macro");
                return;
            }
        }
    }

    // Varsayılan olarak makro örneğini çalıştır
    if let Err(e) = run_macro_example().await {
        eprintln!("Makro örneği çalıştırılırken hata: {}", e);
    }
}
