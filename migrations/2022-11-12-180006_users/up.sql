-- Your SQL goes here
CREATE TABLE users
(
    id          SERIAL PRIMARY KEY,
    username    VARCHAR NOT NULL,
    password    VARCHAR NOT NULL,
    frist_name  VARCHAR NOT NULL
)