use diesel::connection::Connection;
use diesel::prelude::{PgConnection, SqliteConnection};
use interactions::{permissions::PermissionsInteractor, person::PersonInteractor};
use log::{debug, error, info, trace};
use models::Role;
use std::path::Path;
pub mod crypto;
pub mod interactions;
pub mod models;
pub mod schema;

pub enum DbConnection {
    Sqlite(SqliteConnection),
    Pg(PgConnection),
}

pub fn establish_connection(db_url: &str) -> DbConnection {
    let database_url = db_url.to_string();
    trace!("Establishing database connection to: {}", database_url);
    if Path::new(&database_url).exists() {
        trace!("Detected SQLite database");
        return DbConnection::Sqlite(establish_sqlite_connection(&database_url));
    }
    if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
        trace!("Detected PostgreSQL database");
        return DbConnection::Pg(establish_pg_connection(&database_url));
    }
    log::error!("Invalid database URL: {}", database_url);
    panic!("Invalid database URL: {database_url}");
}

fn establish_sqlite_connection(db_url: &str) -> SqliteConnection {
    let database_url = db_url.to_string();
    trace!("Connecting to SQLite database at: {}", database_url);
    match SqliteConnection::establish(&database_url) {
        Ok(conn) => {
            trace!("Successfully connected to SQLite database");
            conn
        }
        Err(e) => {
            log::error!(
                "Failed to connect to SQLite database at {}: {}",
                database_url,
                e
            );
            panic!("Error connecting to {database_url}: {}", e);
        }
    }
}

fn establish_pg_connection(db_url: &str) -> PgConnection {
    let database_url = db_url.to_string();
    trace!("Connecting to PostgreSQL database at: {}", database_url);
    match PgConnection::establish(&database_url) {
        Ok(conn) => {
            trace!("Successfully connected to PostgreSQL database");
            conn
        }
        Err(e) => {
            log::error!(
                "Failed to connect to PostgreSQL database at {}: {}",
                database_url,
                e
            );
            panic!("Error connecting to {database_url}: {}", e);
        }
    }
}

pub fn seed(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Seeding database...");
    let connection = &mut establish_connection(db_url);

    debug!("Creating admin user");
    let person = models::Person::new(
        "Admin",
        "Admin",
        "admin@cpifplosenlaces.com",
        Role::Admin,
        Some(&crypto::to_hash("admin")),
        None,
    );

    let permission = models::Permissions::new(&person.id, true, true, true, true, true);
    match PersonInteractor::new(connection, &person) {
        Ok(_) => info!("Admin user created with ID: {}", person.id),
        Err(e) => {
            error!("Failed to create admin user: {}", e);
            return Err(Box::new(e));
        }
    };

    match PermissionsInteractor::new(connection, &permission) {
        Ok(_) => info!("Admin permissions created for user ID: {}", person.id),
        Err(e) => {
            error!("Failed to create admin permissions: {}", e);
            return Err(Box::new(e));
        }
    };

    info!("Creating 10 regular users");
    for i in 0..10 {
        debug!("Creating user {}", i);
        let person = models::Person::new(
            &format!("User {i}"),
            "User",
            &format!("user{i}@example.com"),
            Role::Alumno,
            Some(&crypto::to_hash(&format!("user{i}"))),
            None,
        );

        let permission = models::Permissions::new(&person.id, true, true, true, true, true);
        match PersonInteractor::new(connection, &person) {
            Ok(_) => debug!("User {} created with ID: {}", i, person.id),
            Err(e) => {
                error!("Failed to create user {}: {}", i, e);
                return Err(Box::new(e));
            }
        };

        match PermissionsInteractor::new(connection, &permission) {
            Ok(_) => debug!("Permissions created for user {} with ID: {}", i, person.id),
            Err(e) => {
                error!("Failed to create permissions for user {}: {}", i, e);
                return Err(Box::new(e));
            }
        };
    }

    info!("Database seeded successfully with 11 users");
    Ok(())
}
