-- Add up migration script here
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS '
  BEGIN
    IF NEW.updated_at IS NULL THEN
    END IF;
        NEW.updated_at := CURRENT_TIMESTAMP;
    RETURN NEW;
  END;
' LANGUAGE plpgsql;

-- local users
CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL CHECK(trim(lower(name)) = name),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- external actors(not contains local users)
CREATE TABLE IF NOT EXISTS actors(
    actor_id UUID PRIMARY KEY,
    actor_url TEXT NOT NULL UNIQUE CHECK(actor_url <> ''),
    preferred_username TEXT NOT NULL,
    inbox_url TEXT NOT NULL UNIQUE CHECK(inbox_url <> ''),
    shared_inbox_url TEXT  CHECK(shared_inbox_url <> ''),

    local_user_id UUID,

    FOREIGN KEY (local_user_id) REFERENCES users(user_id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS actor_rsa_keys(
    actor_id UUID NOT NULL,
    key_url TEXT NOT NULL,
    public_key TEXT NOT NULL CHECK(public_key <> ''),
    private_key TEXT CHECK(private_key <> ''),

    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (actor_id, key_url),

    FOREIGN KEY (actor_id) REFERENCES actors(actor_id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS actor_follows(
    follower_actor_id UUID NOT NULL,
    followed_actor_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (follower_actor_id) REFERENCES actors(actor_id)
    ON UPDATE CASCADE
    ON DELETE CASCADE,

    FOREIGN KEY (followed_actor_id) REFERENCES actors(actor_id)
    ON UPDATE CASCADE
    ON DELETE CASCADE,

    PRIMARY KEY (follower_actor_id, followed_actor_id)
);

CREATE TABLE IF NOT EXISTS notes(
    note_id UUID PRIMARY KEY,
    actor_id UUID NOT NULL,
    content TEXT NOT NULL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
