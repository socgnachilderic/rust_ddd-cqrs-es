-- Add up migration script here
CREATE TABLE IF NOT EXISTS snapshot (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    aggregate_id VARCHAR(36) NOT NULL UNIQUE,
    version BIGINT NOT NULL,
    payload JSONB NOT NULL
);
CREATE TABLE IF NOT EXISTS post_event_store (
    id UUID NOT NULL DEFAULT gen_random_uuid(),
    aggregate_id VARCHAR(36) NOT NULL,
    version BIGINT NOT NULL,
    event_type VARCHAR NOT NULL,
    payload JSONB NOT NULL,
    occurred_on TIMESTAMPTZ NOT NULL
);
CREATE TABLE IF NOT EXISTS post_projection (
    id VARCHAR(36) PRIMARY KEY,
    title VARCHAR(100) NOT NULL,
    content TEXT NOT NULL,
    version BIGINT NOT NULL,
    created_on TIMESTAMPTZ NOT NULL,
    updated_on TIMESTAMPTZ NOT NULL
);