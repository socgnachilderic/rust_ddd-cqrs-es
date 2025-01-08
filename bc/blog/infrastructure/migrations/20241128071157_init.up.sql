-- Add up migration script here
DROP TABLE IF EXISTS post;

CREATE TABLE post (
    id varchar(36) PRIMARY KEY,
    title varchar(100) NOT NULL UNIQUE,
    content text NOT NULL
);

CREATE TABLE comment (
    id varchar(36) PRIMARY KEY,
    body text NOT NULL,
    post_id varchar(36) NOT NULL,
    CONSTRAINT fk_post FOREIGN KEY (post_id) REFERENCES post(id)
)
