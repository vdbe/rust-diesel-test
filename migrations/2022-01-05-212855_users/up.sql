-- Your SQL goes here
CREATE TABLE users
(
    id         SERIAL PRIMARY KEY,
    username   VARCHAR NOT NULL,
    password   VARCHAR NOT NULL,
    first_name VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL
);
