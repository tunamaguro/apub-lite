-- Add up migration script here
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS '
  BEGIN
    IF NEW.updated_at IS NULL THEN
    END IF;
        NEW.updated_at := CURRENT_TIMESTAMP;
    RETURN NEW;
  END;
' LANGUAGE plpgsql;

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL CHECK(trim(lower(name)) = name),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE TRIGGER set_users_updated_at
    BEFORE UPDATE ON users FOR EACH ROW
    EXECUTE PROCEDURE set_updated_at();

CREATE TABLE IF NOT EXISTS user_rsa_keys(
    user_id UUID PRIMARY KEY,
    private_key TEXT NOT NULL CHECK(private_key <> ''),
    public_key TEXT NOT NULL CHECK(public_key <> ''),

    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (user_id) REFERENCES users(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
);

CREATE TRIGGER set_user_rsa_key_updated_at
    BEFORE UPDATE ON user_rsa_keys FOR EACH ROW
    EXECUTE PROCEDURE set_updated_at();


CREATE TABLE IF NOT EXISTS followers(
    user_id UUID NOT NULL,
    actor_url TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (user_id) REFERENCES users(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE,

    PRIMARY KEY (user_id,actor_url)
);

CREATE TABLE IF NOT EXISTS notes(
    note_id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER set_notes_updated_at
    BEFORE UPDATE ON notes FOR EACH ROW
    EXECUTE PROCEDURE set_updated_at();