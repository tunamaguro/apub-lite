-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL CHECK(trim(lower(name)) = name)
);