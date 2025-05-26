-- Create password reset tokens table
CREATE TABLE password_reset_tokens (
    id CHAR(36) PRIMARY KEY NOT NULL,
    email VARCHAR(100) NOT NULL,
    token VARCHAR(64) NOT NULL UNIQUE,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);