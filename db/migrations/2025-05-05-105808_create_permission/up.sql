CREATE TABLE permissions (
    id CHAR(32) PRIMARY KEY NOT NULL,
    person CHAR(32) NOT NULL,
    dashboard BOOLEAN NOT NULL DEFAULT FALSE,
    see_self_history BOOLEAN NOT NULL DEFAULT FALSE,
    see_others_history BOOLEAN NOT NULL DEFAULT FALSE,
    admin_panel BOOLEAN NOT NULL DEFAULT FALSE,
    edit_permissions BOOLEAN NOT NULL DEFAULT FALSE,
    FOREIGN KEY (person) REFERENCES Person(id)
);
