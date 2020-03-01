ALTER TABLE confirmed_users
DROP CONSTRAINT fk_token_id,
DROP COLUMN token_id;

DROP TABLE tokens;
