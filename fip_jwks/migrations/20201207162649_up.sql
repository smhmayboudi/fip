-- Add migration script here
CREATE TABLE IF NOT EXISTS jwks (
    id VARCHAR(36) NOT NULL PRIMARY KEY,
    public_key TEXT NOT NULL,
    private_key TEXT NOT NULL
)