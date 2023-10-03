set client_encoding = 'UTF8';

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    salt VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    session_token VARCHAR(255) UNIQUE
);

CREATE TABLE IF NOT EXISTS contents (
    content_id SERIAL PRIMARY KEY,
    -- 0: 仕事, 1: 仕事以外
    content_kind INTEGER NOT NULL CHECK (content_kind IN (0, 1)),
    content VARCHAR(1023) NOT NULL
);

CREATE TABLE IF NOT EXISTS gongzuo (
    id SERIAL PRIMARY KEY,
    user_id SERIAL NOT NULL REFERENCES users(id),
    content_id SERIAL NOT NULL REFERENCES contents(content_id),
    started_at TIMESTAMP NOT NULL,
     -- NULL: not ended yet
    ended_at TIMESTAMP
);