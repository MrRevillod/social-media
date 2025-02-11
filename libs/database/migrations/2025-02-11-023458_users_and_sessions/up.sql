-- Your SQL goes here

create table users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password TEXT NOT NULL,
    validated BOOLEAN NOT NULL DEFAULT FALSE
);

create table sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    token TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

ALTER TABLE users
ADD COLUMN created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
ADD COLUMN updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL;

ALTER TABLE sessions
ADD COLUMN created_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL,
ADD COLUMN updated_at TIMESTAMP WITH TIME ZONE DEFAULT now() NOT NULL;

SELECT diesel_manage_updated_at('users');
SELECT diesel_manage_updated_at('sessions');

