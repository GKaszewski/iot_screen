CREATE TABLE IF NOT EXISTS weather (
    id INTEGER PRIMARY KEY,
    time TEXT NOT NULL,
    interval INTEGER NOT NULL,
    temperature REAL NOT NULL
)