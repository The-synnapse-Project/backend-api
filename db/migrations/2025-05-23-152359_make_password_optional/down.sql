-- Revert password_hash back to NOT NULL
ALTER TABLE person ALTER COLUMN password_hash SET NOT NULL;