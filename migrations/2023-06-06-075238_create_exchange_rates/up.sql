-- Your SQL goes here
CREATE TABLE exchange_rates (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    base TEXT NOT NULL,
    gbp REAL NOT NULL,
    updated_at TEXT NOT NULL DEFAULT CURRENT_DATE
);