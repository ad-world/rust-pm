-- Your SQL goes here
CREATE TABLE vault_password_entry (
    id SERIAL PRIMARY KEY,
    vault_id INT NOT NULL,
    service VARCHAR NOT NULL,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (vault_id) REFERENCES vault(id)
);