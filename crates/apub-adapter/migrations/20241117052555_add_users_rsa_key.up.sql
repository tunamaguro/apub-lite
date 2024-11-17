-- Add up migration script here
CREATE TABLE IF NOT EXISTS user_rsa_keys(
    user_id UUID PRIMARY KEY,
    private_key TEXT NOT NULL CHECK(private_key <> ''),
    public_key TEXT NOT NULL CHECK(public_key <> ''),

    FOREIGN KEY (user_id) REFERENCES users(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE
);