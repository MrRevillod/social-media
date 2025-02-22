-- Add migration script here

ALTER TABLE sessions ADD COLUMN user_agent TEXT;
ALTER TABLE sessions ADD COLUMN ip_address TEXT;
ALTER TABLE sessions ADD COLUMN active BOOLEAN;
ALTER TABLE sessions ADD COLUMN expires_at TIMESTAMPTZ;
