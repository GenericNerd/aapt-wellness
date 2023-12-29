-- Add migration script here
CREATE TABLE users (
    iv VARCHAR(64) NOT NULL,
    name VARCHAR(128) NOT NULL,
    email VARCHAR(512) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);