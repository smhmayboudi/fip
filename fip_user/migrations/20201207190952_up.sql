-- Add migration script here
CREATE TABLE IF NOT EXISTS user (
    avatar TEXT NOT NULL,
    biography TEXT NOT NULL,
    birth_date INTEGER NOT NULL,
    cellphone TEXT NOT NULL,
    email TEXT NOT NULL,
    first_name TEXT NOT NULL,
    gender TEXT NOT NULL,
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    language_code TEXT NOT NULL,
    last_name TEXT NOT NULL,
    password TEXT NOT NULL,
    registered_at INTEGER NOT NULL,
    username TEXT NOT NULL
);
