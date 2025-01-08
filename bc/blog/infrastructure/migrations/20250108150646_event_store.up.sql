-- Add up migration script here
CREATE TABLE IF NOT EXISTS post_event_store (
    id SERIAL PRIMARY KEY,
    aggregate_id VARCHAR NOT NULL,
    version BIGINT NOT NULL,
    event_type VARCHAR NOT NULL,
    payload JSONB NOT NULL,
    occurred_on TIMESTAMPTZ NOT NULL
);
CREATE TABLE IF NOT EXISTS post_projection (
    id varchar(36) PRIMARY KEY,
    title varchar(100) NOT NULL UNIQUE,
    content text NOT NULL
);