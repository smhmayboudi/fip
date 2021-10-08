-- Add migration script here
CREATE TABLE IF NOT EXISTS rt (
    claims_aud TEXT NOT NULL,
    claims_exp INTEGER NOT NULL,
    claims_iat INTEGER NOT NULL,
    claims_iss TEXT NOT NULL,
    claims_jti VARCHAR(36) NOT NULL PRIMARY KEY,
    claims_nbf INTEGER NOT NULL,
    claims_sub VARCHAR(36) NOT NULL,
    token_blocked BOOLEAN NOT NULL,
    token_blocked_description TEXT NOT NULL
)
