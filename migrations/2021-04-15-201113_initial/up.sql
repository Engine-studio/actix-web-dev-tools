-- Your SQL goes here
CREATE TABLE Auth (
    id BIGSERIAL PRIMARY KEY,
    login VARCHAR NOT NULL,
    auth_type VARCHAR NOT NULL,
    roles TEXT[] NOT NULL 
);
