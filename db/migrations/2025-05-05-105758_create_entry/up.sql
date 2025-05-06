-- Your SQL goes here
CREATE TABLE entries (
    id CHAR(32) PRIMARY KEY NOT NULL,
    person_id CHAR(32) NOT NULL,
    instant TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    action VARCHAR(100) NOT NULL,
    FOREIGN KEY (person_id) REFERENCES Person(id)
);