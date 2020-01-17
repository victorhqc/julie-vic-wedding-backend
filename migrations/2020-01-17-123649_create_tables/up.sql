-- Your SQL goes here

CREATE TABLE users (
  id UUID PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  last_name TEXT NULL,
  email TEXT UNIQUE NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NULL
);

CREATE TABLE tables (
  id UUID PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  alias TEXT NULL
);

CREATE TABLE confirmed_users (
  user_id UUID NOT NULL,
  table_id UUID NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL,
  updated_at TIMESTAMP WITH TIME ZONE NULL,
  PRIMARY KEY (user_id),
  CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
  CONSTRAINT fk_table_id FOREIGN KEY (table_id) REFERENCES tables (id) ON DELETE SET NULL
);
