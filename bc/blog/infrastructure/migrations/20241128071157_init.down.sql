-- Add down migration script here
DROP TABLE IF EXISTS snapshot;
DROP TABLE IF EXISTS post_projection;
DROP TABLE IF EXISTS post_event_store;