-- Add down migration script here
DROP TRIGGER IF EXISTS set_users_updated_at ON notes;
DROP TABLE IF EXISTS notes;


DROP TRIGGER IF EXISTS set_user_rsa_key_updated_at ON user_rsa_keys;
DROP TABLE IF EXISTS user_rsa_keys;

DROP TRIGGER IF EXISTS set_notes_updated_at ON users;
DROP TABLE IF EXISTS users;

