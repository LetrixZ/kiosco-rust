// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::{error, info, LevelFilter};
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteRow, FromRow, Pool, Row, Sqlite, SqlitePool};
use std::fs::create_dir_all;
use tauri::{Manager, State};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget};
use tokio::{runtime::Runtime, sync::Mutex};

const LOG_TARGETS: [LogTarget; 2] = [LogTarget::Stdout, LogTarget::LogDir];

pub struct AppState {
  db: Mutex<Pool<Sqlite>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Product {
  #[serde(default)]
  id: i64,
  name: String,
  description: Option<String>,
  barcode: String,
  price: f64,
  cost: f64,
  stock: i64,
  #[serde(default)]
  created_at: String,
  #[serde(default)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
struct InvoiceLine {
  #[serde(default)]
  id: i64,
  name: String,
  quantity: i64,
  price: f64,
  #[serde(default)]
  invoice: Option<Invoice>,
  product: Option<Product>,
  #[serde(default)]
  created_at: String,
  #[serde(default)]
  updated_at: String,
}

impl FromRow<'_, SqliteRow> for InvoiceLine {
  fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
    let invoice: Option<Invoice> = match row.try_get::<String, &str>("invoice") {
      Ok(str) => serde_json::from_str(&str).unwrap_or(None),
      Err(_) => None,
    };

    let product: Option<Product> = match row.try_get::<String, &str>("product") {
      Ok(str) => serde_json::from_str(&str).unwrap_or(None),
      Err(_) => None,
    };

    Ok(Self {
      id: row.get("id"),
      name: row.get("name"),
      quantity: row.get("quantity"),
      price: (row.get::<i64, &str>("price") as f64) / 100.0,
      invoice,
      product,
      created_at: row.get("created_at"),
      updated_at: row.get("updated_at"),
    })
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Invoice {
  #[serde(default)]
  id: i64,
  total: f64,
  lines: Option<Vec<InvoiceLine>>,
  #[serde(default)]
  created_at: String,
  #[serde(default)]
  updated_at: String,
}

impl FromRow<'_, SqliteRow> for Invoice {
  fn from_row(row: &'_ SqliteRow) -> Result<Self, sqlx::Error> {
    let lines: Option<Vec<InvoiceLine>> = match row.try_get::<String, &str>("invoice_lines") {
      Ok(str) => match serde_json::from_str::<Vec<InvoiceLine>>(&str) {
        Ok(mut lines) => {
          for line in &mut lines {
            if let Some(ref mut product) = line.product {
              product.price /= 100.0;
              product.cost /= 100.0;
            }

            line.price /= 100.0;
          }
          Some(lines)
        }
        Err(_) => None,
      },
      Err(_) => None,
    };

    Ok(Self {
      id: row.get("id"),
      total: (row.get::<i64, &str>("total") as f64) / 100.0,
      lines,
      created_at: row.get("created_at"),
      updated_at: row.get("updated_at"),
    })
  }
}

fn main() {
  let rt = Runtime::new().unwrap();

  tauri::Builder::default()
    .setup(move |app| {
      let path = app.path_resolver().app_data_dir().unwrap();

      match create_dir_all(&path) {
        Ok(_) => {
          let db_url: &str = &format!("sqlite://{}/sqlite.db", path.to_str().unwrap());

          rt.block_on(async {
            if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
              match Sqlite::create_database(db_url).await {
                Ok(_) => (),
                Err(error) => panic!("Database creation error: {}", error),
              }
            }

            let db = SqlitePool::connect(db_url).await.unwrap();
            let migrations = app
              .path_resolver()
              .resource_dir()
              .unwrap()
              .as_path()
              .join("migrations");
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
        }
        Err(e) => {
          println!("Failed to create data directory: {:?}", e);
        }
      };

      Ok(())
    })
    .plugin(
      tauri_plugin_log::Builder::default()
        .targets(LOG_TARGETS)
        .level(LevelFilter::Info)
        .with_colors(ColoredLevelConfig::default())
        .build(),
    )
    .invoke_handler(tauri::generate_handler![
      search,
      get_products,
      create_product,
      save_product,
      get_invoices,
      create_invoice
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
async fn search(query: String, state: State<'_, AppState>) -> Result<Vec<Product>, String> {
  let db = state.db.lock().await;

  match sqlx::query_as::<_, Product>(
    r#"SELECT * FROM products WHERE name LIKE $1 OR barcode = $2 LIMIT 25"#,
  )
  .bind(format!("%{}%", query))
  .bind(&query)
  .fetch_all(&*db)
  .await
  {
    Ok(products) => {
      info!("Found {} products", products.len());

      Ok(products)
    }
    Err(e) => {
      error!("Failed to search products: {:?}\nQuery: {}", e, query);

      Err("Ocurrió un error al intentar obtener los productos.".to_string())
    }
  }
}

#[tauri::command]
async fn get_products(state: State<'_, AppState>) -> Result<Vec<Product>, String> {
  let db = state.db.lock().await;

  info!("Fetching products");

  match sqlx::query_as::<_, Product>(r#"SELECT * FROM products"#)
    .fetch_all(&*db)
    .await
  {
    Ok(products) => {
      info!("Found {} products", products.len());

      Ok(products)
    }
    Err(e) => {
      error!("Failed to get products: {:?}", e);

      Err("Ocurrió un error al intentar obtener los productos.".to_string())
    }
  }
}

#[tauri::command]
async fn create_product(product: Product, state: State<'_, AppState>) -> Result<(), String> {
  let db = state.db.lock().await;

  info!("Creating new product");

  match sqlx::query(
    r#"INSERT INTO products(
      name, description, barcode, cost, price, stock
    ) 
    VALUES
      ($1, $2, $3, $4, $5, $6)
      "#,
  )
  .bind(&product.name)
  .bind(&product.description)
  .bind(&product.barcode)
  .bind((product.cost * 100.0) as i64)
  .bind((product.price * 100.0) as i64)
  .bind(product.stock)
  .execute(&*db)
  .await
  {
    Ok(_) => {
      info!("Product created succefully");
      Ok(())
    }
    Err(e) => {
      error!("Failed to create product: {:?}\nData: {:?}", e, product);

      Err("Ocurrió un error al intentar crear un nuevo producto.".to_string())
    }
  }
}

#[tauri::command]
async fn save_product(product: Product, state: State<'_, AppState>) -> Result<(), String> {
  let db = state.db.lock().await;

  info!("Saving product \"{}\"s", product.name);

  match sqlx::query(
    r#"UPDATE 
        products 
      SET 
        name = $2, 
        description = $3, 
        barcode = $4, 
        cost = $5, 
        price = $6, 
        stock = $7 
      WHERE 
        id = $1
      "#,
  )
  .bind(product.id)
  .bind(&product.name)
  .bind(&product.description)
  .bind(&product.barcode)
  .bind((product.cost * 100.0) as i64)
  .bind((product.price * 100.0) as i64)
  .bind(product.stock)
  .execute(&*db)
  .await
  {
    Ok(_) => {
      info!("Product \"{}\" saved correctly", product.name);

      Ok(())
    }
    Err(e) => {
      error!(
        "Product saving failed for \"{}\": {:?}\nData: {:#?}",
        product.name, e, product
      );

      Err(format!(
        "Ocurrió un error en el guardado del producto \"{}\"",
        product.name
      ))
    }
  }
}

#[tauri::command]
async fn _get_invoice_lines(state: State<'_, AppState>) -> Result<Vec<InvoiceLine>, String> {
  let db = state.db.lock().await;

  sqlx::query_as::<_, InvoiceLine>(
    r#"SELECT 
      l.id, 
      l.name, 
      l.quantity, 
      l.price, 
      JSON_OBJECT(
        'id', i.id, 'total', i.total, 'created_at', 
        i.created_at, 'updated_at', i.updated_at
      ) as invoice, 
      JSON_OBJECT(
        'id', p.id, 'name', p.name, 'description', 
        p.description, 'barcode', p.barcode, 
        'price', p.price, 'cost', p.cost, 
        'stock', p.stock, 'created_at', p.created_at, 
        'updated_at', p.updated_at
      ) as product, 
      l.created_at, 
      l.updated_at 
      FROM 
        invoice_lines l 
        LEFT JOIN invoices i ON i.id = l.invoice_id 
        LEFT JOIN products p ON p.id = l.product_id;
    "#,
  )
  .fetch_all(&*db)
  .await
  .map_err(|_| "Failed to get invoice lines".to_string())
}

#[tauri::command]
async fn get_invoices(state: State<'_, AppState>) -> Result<Vec<Invoice>, String> {
  let db = state.db.lock().await;

  info!("Fetching invoices");

  match sqlx::query_as::<_, Invoice>(
    r#"SELECT 
        i.id, 
        i.total, 
        JSON_GROUP_ARRAY(
          JSON_OBJECT(
            'id', 
            l.id, 
            'name', 
            l.name, 
            'quantity', 
            l.quantity, 
            'price', 
            l.price, 
            'product', 
            JSON_OBJECT(
              'id', p.id, 'name', p.name, 'description', 
              p.description, 'barcode', p.barcode, 
              'price', p.price, 'cost', p.cost, 
              'stock', p.stock, 'created_at', p.created_at, 
              'updated_at', p.updated_at
            ),
            'created_at', l.created_at, 
            'updated_at', l.updated_at
          )
        ) as invoice_lines, 
        i.created_at, 
        i.updated_at 
      FROM 
        invoices i 
        LEFT JOIN invoice_lines l ON l.invoice_id = i.id 
        LEFT JOIN products p ON p.id = l.product_id 
      GROUP BY 
        i.id;
      "#,
  )
  .fetch_all(&*db)
  .await
  {
    Ok(invoices) => {
      info!("Found {} invoices", invoices.len());

      Ok(invoices)
    }
    Err(e) => {
      error!("Failed to get invoices: {:?}", e);

      Err("Ocurrió un error al intentar obtener las facturas.".to_string())
    }
  }
}

#[tauri::command]
async fn create_invoice(invoice: Invoice, state: State<'_, AppState>) -> Result<(), String> {
  let db = state.db.lock().await;

  let result = sqlx::query(r#"INSERT INTO invoices(total) VALUES($1)"#)
    .bind((invoice.total * 100.0) as i64)
    .execute(&*db)
    .await
    .map_err(|e| {
      error!("Failed to create invoice: {:?}\nData: {:#?}", e, invoice);

      "Ocurrió un error al crear la factura".to_string()
    })?;

  let invoice_id = result.last_insert_rowid();

  for line in invoice.lines.unwrap() {
    let product_id: Option<i64> = match line.product {
      Some(product) => {
        match sqlx::query(r#"UPDATE products SET stock = stock - $2 WHERE id = $1"#)
          .bind(product.id)
          .bind(line.quantity)
          .execute(&*db)
          .await
        {
          Ok(_) => {
            info!("Product \"{}\" stock updated correctly", product.name);
          }
          Err(e) => {
            error!(
              "Error while updating product stock: {:?}\nData: {:#?}",
              e, product
            );
          }
        }

        Some(product.id)
      }
      None => None,
    };

    match sqlx::query(
      r#"INSERT INTO invoice_lines(
            name, quantity, price, product_id, 
            invoice_id
          ) 
          VALUES 
            ($1, $2, $3, $4, $5)      
          "#,
    )
    .bind(line.name)
    .bind(line.quantity)
    .bind((line.price * 100.0) as i64)
    .bind(product_id)
    .bind(invoice_id)
    .execute(&*db)
    .await
    {
      Ok(_) => {
        info!("Line for invoice \"{}\" inserted correctly", invoice_id);
      }
      Err(e) => {
        error!(
          "Failed to insert line for invoice \"{}\": {:?}",
          invoice_id, e
        )
      }
    }
  }

  Ok(())
}
