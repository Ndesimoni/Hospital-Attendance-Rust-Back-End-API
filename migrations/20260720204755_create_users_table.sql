-- Add migration script here
CREATE TABLE users
(
    id SERIAL PRIMARY KEY,
    email VARCHAR
    (255) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)