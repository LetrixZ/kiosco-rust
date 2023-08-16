// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteRow, FromRow, Pool, Row, Sqlite, SqlitePool};
use tauri::{Manager, State};
use tokio::{runtime::Runtime, sync::Mutex};

pub struct AppState {
  db: Mutex<Pool<Sqlite>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Product {
  id: i64,
  name: String,
  description: Option<String>,
  barcode: String,
  price: f64,
  cost: f64,
  stock: i64,
  created_at: String,
  updated_at: String,
}

impl FromRow<'_, SqliteRow> for Product {
  fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
    Ok(Self {
      id: row.get("id"),
      name: row.get("name"),
      description: row.get("description"),
      barcode: row.get("barcode"),
      price: (row.get::<i64, &str>("price") as f64) / 100.0,
      cost: (row.get::<i64, &str>("cost") as f64) / 100.0,
      stock: row.get("stock"),
      created_at: row.get("created_at"),
      updated_at: row.get("updated_at"),
    })
  }
}

fn main() {
  let rt = Runtime::new().unwrap();

  tauri::Builder::default()
    .setup(move |app| {
      let db_url: &str = &format!(
        "sqlite://{}/sqlite.db",
        app
          .path_resolver()
          .app_data_dir()
          .unwrap()
          .to_str()
          .unwrap()
      );

      rt.block_on(async {
        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
          match Sqlite::create_database(db_url).await {
            Ok(_) => (),
            Err(error) => panic!("Database creation error: {}", error),
          }
        }

        let db = SqlitePool::connect(db_url).await.unwrap();
        let migrations = std::path::Path::new("./migrations");
        let migration_results = sqlx::migrate::Migrator::new(migrations)
          .await
          .unwrap()
          .run(&db)
          .await;
        match migration_results {
          Ok(_) => (),
          Err(error) => panic!("Migration error: {:?}", error),
        }

        app.manage(AppState { db: Mutex::new(db) })
      });
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![search, get_products, save_product])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn search(query: String, state: State<'_, AppState>) -> Result<Vec<Product>, String> {
  let db = state.db.lock().await;

  let result: Vec<Product> =
    sqlx::query_as::<_, Product>(r#"SELECT * FROM products WHERE name LIKE $1 LIMIT 25"#)
      .bind(format!("%{}%", query))
      .fetch_all(&*db)
      .await
      .map_err(|e| {
        println!("{:?}", e);
        "Failed to get products".to_string()
      })?;

  Ok(result)
}

#[tauri::command]
async fn get_products(state: State<'_, AppState>) -> Result<Vec<Product>, String> {
  let db = state.db.lock().await;

  Ok(
    sqlx::query_as::<_, Product>(r#"SELECT * FROM products"#)
      .fetch_all(&*db)
      .await
      .map_err(|_| "Failed to get products".to_string())?,
  )
}

#[tauri::command]
async fn save_product(product: Product, state: State<'_, AppState>) -> Result<(), String> {
  let db = state.db.lock().await;

  sqlx::query(r#"UPDATE products SET name = $2, description = $3, barcode = $4, cost = $5, price = $6, stock = $7 WHERE id = $1"#)
    .bind(product.id)
    .bind(product.name)
    .bind(product.description)
    .bind(product.barcode)
    .bind((product.cost * 100.0) as i64)
    .bind((product.price * 100.0) as i64)
    .bind(product.stock)
    .execute(&*db)
    .await
    .map_err(|_| "Failed to update product".to_string())?;

  Ok(())
}
