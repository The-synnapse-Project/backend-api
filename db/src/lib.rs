use diesel::connection::Connection;
use diesel::prelude::{PgConnection, SqliteConnection};
use interactions::{Permissions, Person};
use std::env;
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
    let database_url = env::var("DATABASE_URL").unwrap_or(db_url.to_string());
    if Path::new(&database_url).exists() {
        return DbConnection::Sqlite(establish_sqlite_connection(&database_url));
    }
    if database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
        return DbConnection::Pg(establish_pg_connection(&database_url));
    }
    panic!("Invalid database URL: {database_url}");
}

fn establish_sqlite_connection(db_url: &str) -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").unwrap_or(db_url.to_string());
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

fn establish_pg_connection(db_url: &str) -> PgConnection {
    let database_url = env::var("DATABASE_URL").unwrap_or(db_url.to_string());
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

pub fn seed(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let connection = &mut establish_connection(db_url);
    let person = models::Person::new(
        "Admin",
        "Admin",
        "admin@cpiftlosenlaces.com",
        &crypto::to_hash("admin"),
    );

    let permission = models::Permissions::new(&person.id, true, true, true, true, true);
    Person::new(connection, &person)?;
    Permissions::new(connection, &permission)?;

    Ok(())
}
