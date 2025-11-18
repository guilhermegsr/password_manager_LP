PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS user
(
    id            BLOB PRIMARY KEY,
    username      TEXT UNIQUE NOT NULL,
    password_hash BLOB        NOT NULL,
    created_at    TEXT        NOT NULL,
    updated_at    TEXT        NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_user_username
    ON user (username);

CREATE TABLE IF NOT EXISTS vault
(
    id               BLOB PRIMARY KEY,
    user_id          BLOB NOT NULL,
    vault_key_cipher BLOB NOT NULL,
    created_at       TEXT NOT NULL,
    updated_at       TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_vault_user
    ON vault (user_id);

CREATE TABLE IF NOT EXISTS credential
(
    id              BLOB PRIMARY KEY,
    vault_id        BLOB NOT NULL,
    name            TEXT NOT NULL,
    username        TEXT,
    url             TEXT,
    notes           BLOB,
    password_cipher BLOB,
    created_at      TEXT NOT NULL,
    updated_at      TEXT NOT NULL,
    FOREIGN KEY (vault_id) REFERENCES vault (id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_credential_vault
    ON credential (vault_id);

CREATE INDEX IF NOT EXISTS idx_credential_name
    ON credential (name);