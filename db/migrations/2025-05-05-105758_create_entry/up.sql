-- Your SQL goes here
CREATE TABLE entries (
    id CHAR(36) PRIMARY KEY NOT NULL,
    person_id CHAR(36) NOT NULL,
    instant TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    action VARCHAR(100) NOT NULL,
    FOREIGN KEY (person_id) REFERENCES Person (id)
);