-- Your SQL goes here
CREATE TABLE pending_files (
    id BIGSERIAL PRIMARY KEY,
    url VARCHAR NOT NULL UNIQUE,
    upload TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
