CREATE TABLE products(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL, 
  description TEXT NULL,
  barcode TEXT NULL,
  price INTEGER NOT NULL,
  cost INTEGER NOT NULL,
  stock INTEGER NOT NULL DEFAULT 0,
  created_at TEXT,
  updated_at TEXT
);
CREATE TABLE invoices(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  total INTEGER NOT NULL,
  created_at TEXT,
  updated_at TEXT
);
CREATE TABLE invoice_lines (
	id	INTEGER,
	name	TEXT NOT NULL,
	quantity	INTEGER NOT NULL,
	price	INTEGER NOT NULL,
	invoice_id	INTEGER NOT NULL,
	product_id	INTEGER,
  created_at	TEXT,
	updated_at	TEXT,
	PRIMARY KEY(id AUTOINCREMENT),
	FOREIGN KEY(product_id) REFERENCES products(id) ON DELETE SET NULL,
	FOREIGN KEY(invoice_id) REFERENCES invoices(id) ON DELETE CASCADE
);
CREATE TRIGGER insert_timestamp_trigger_products
AFTER INSERT ON products
BEGIN
  UPDATE products SET created_at = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW'), updated_at = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW') WHERE id = NEW.id;
END;
CREATE TRIGGER update_timestamp_trigger_products
AFTER UPDATE ON products
BEGIN
  UPDATE products SET updated_at = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW') WHERE id = NEW.id;
END;
CREATE TRIGGER insert_timestamp_trigger_invoices
AFTER INSERT ON invoices
BEGIN
  UPDATE invoices SET created_at = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW'), updated_at = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW') WHERE id = NEW.id;
END;
CREATE TRIGGER update_timestamp_trigger_invoices
AFTER UPDATE ON invoices
BEGIN
  UPDATE invoices SET updated_at = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW') WHERE id = NEW.id;
END;
CREATE TRIGGER insert_timestamp_trigger_invoice_lines
AFTER INSERT ON invoice_lines
BEGIN
  UPDATE invoice_lines SET created_at = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW'), updated_at = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW') WHERE id = NEW.id;
END;
CREATE TRIGGER update_timestamp_trigger_invoice_lines
AFTER UPDATE ON invoice_lines
BEGIN
  UPDATE invoice_lines SET updated_at = STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW') WHERE id = NEW.id;
END;