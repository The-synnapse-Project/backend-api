use diesel::prelude::*;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection(db_url: &str) -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").unwrap_or(db_url.to_string());
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

pub fn create_person(conn: &mut SqliteConnection, person: &models::Person) -> QueryResult<usize> {
    use crate::schema::person;
    diesel::insert_into(person::table)
        .values(person)
        .execute(conn)
}
