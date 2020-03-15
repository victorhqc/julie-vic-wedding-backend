CREATE TYPE available_language as ENUM ('en', 'es');

ALTER TABLE users
ADD COLUMN language available_language NOT NULL DEFAULT 'en';
