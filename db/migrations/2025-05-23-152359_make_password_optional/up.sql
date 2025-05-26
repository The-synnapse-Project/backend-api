-- Make password_hash nullable to support Google users without passwords
ALTER TABLE person ALTER COLUMN password_hash DROP NOT NULL;