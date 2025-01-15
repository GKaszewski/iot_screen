CREATE TABLE api_keys (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    service_name TEXT NOT NULL,
    api_key TEXT NOT NULL
);