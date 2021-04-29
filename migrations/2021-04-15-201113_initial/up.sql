-- Your SQL goes here
CREATE TABLE Auth (
    id uuid DEFAULT uuid_generate_v4 (),
    login VARCHAR NOT NULL,
    auth_type VARCHAR NOT NULL,
    roles TEXT[] NOT NULL 
    PRIMARY KEY (id)
);
