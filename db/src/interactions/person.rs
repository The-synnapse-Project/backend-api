use crate::DbConnection;
use crate::models;
use diesel::prelude::*;
use log::{debug, error, info};

pub struct PersonInteractor {}

impl PersonInteractor {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(conn: &mut DbConnection, person: &models::Person) -> QueryResult<usize> {
        use crate::schema::person;
        debug!("Creating new person: {} <{}>", person.name, person.email);
        let result = match conn {
            DbConnection::Sqlite(conn) => diesel::insert_into(person::table)
                .values(person)
                .execute(conn),
            DbConnection::Pg(conn) => diesel::insert_into(person::table)
                .values(person)
                .execute(conn),
        };

        match &result {
            Ok(rows) => info!(
                "Created person with ID: {}, affected {} rows",
                person.id, rows
            ),
            Err(e) => error!("Failed to create person: {}", e),
        }

        result
    }

    pub fn get(conn: &mut DbConnection) -> QueryResult<Vec<models::Person>> {
        use crate::schema::person::dsl::*;
        debug!("Retrieving all persons");
        let result = match conn {
            DbConnection::Sqlite(conn) => person.load::<models::Person>(conn),
            DbConnection::Pg(conn) => person.load::<models::Person>(conn),
        };

        match &result {
            Ok(persons) => debug!("Retrieved {} persons", persons.len()),
            Err(e) => error!("Failed to retrieve persons: {}", e),
        }

        result
    }

    pub fn get_by_id(conn: &mut DbConnection, p_id: &str) -> QueryResult<models::Person> {
        use crate::schema::person::dsl::*;
        debug!("Retrieving person with ID: {}", p_id);
        let result = match conn {
            DbConnection::Sqlite(conn) => person.filter(id.eq(p_id)).first::<models::Person>(conn),
            DbConnection::Pg(conn) => person.filter(id.eq(p_id)).first::<models::Person>(conn),
        };

        match &result {
            Ok(p) => debug!("Retrieved person: {} <{}>", p.name, p.email),
            Err(e) => error!("Failed to retrieve person with ID {}: {}", p_id, e),
        }

        result
    }

    pub fn get_by_email(conn: &mut DbConnection, req_email: &str) -> QueryResult<models::Person> {
        use crate::schema::person::dsl::*;
        debug!("Retrieving person with email: {}", req_email);
        let result = match conn {
            DbConnection::Sqlite(conn) => person
                .filter(email.eq(req_email))
                .first::<models::Person>(conn),
            DbConnection::Pg(conn) => person
                .filter(email.eq(req_email))
                .first::<models::Person>(conn),
        };

        match &result {
            Ok(p) => debug!("Retrieved person with ID: {} and name: {}", p.id, p.name),
            Err(e) => error!("Failed to retrieve person with email {}: {}", req_email, e),
        }

        result
    }

    pub fn get_by_google_id(conn: &mut DbConnection, g_id: &str) -> QueryResult<models::Person> {
        use crate::schema::person::dsl::*;
        debug!("Retrieving person with Google ID: {}", g_id);
        let result = match conn {
            DbConnection::Sqlite(conn) => person
                .filter(google_id.eq(g_id))
                .first::<models::Person>(conn),
            DbConnection::Pg(conn) => person
                .filter(google_id.eq(g_id))
                .first::<models::Person>(conn),
        };

        match &result {
            Ok(p) => debug!("Retrieved person with ID: {} and name: {}", p.id, p.name),
            Err(e) => error!("Failed to retrieve person with Google ID {}: {}", g_id, e),
        }

        result
    }

    pub fn update(
        conn: &mut DbConnection,
        p_id: &str,
        person_changes: &models::Person,
    ) -> QueryResult<usize> {
        use crate::schema::person::dsl::*;
        info!(
            "Updating person with ID: {} to name: {} and email: {}",
            p_id, person_changes.name, person_changes.email
        );

        let result = match conn {
            DbConnection::Sqlite(conn) => diesel::update(person.filter(id.eq(p_id)))
                .set(person_changes)
                .execute(conn),
            DbConnection::Pg(conn) => diesel::update(person.filter(id.eq(p_id)))
                .set(person_changes)
                .execute(conn),
        };

        match &result {
            Ok(rows) => info!("Updated person with ID: {}, affected {} rows", p_id, rows),
            Err(e) => error!("Failed to update person with ID {}: {}", p_id, e),
        }

        result
    }

    pub fn delete(conn: &mut DbConnection, p_id: &str) -> QueryResult<usize> {
        use crate::schema::person::dsl::*;
        info!("Deleting person with ID: {}", p_id);

        let result = match conn {
            DbConnection::Sqlite(conn) => diesel::delete(person.filter(id.eq(p_id))).execute(conn),
            DbConnection::Pg(conn) => diesel::delete(person.filter(id.eq(p_id))).execute(conn),
        };

        match &result {
            Ok(rows) => info!("Deleted person with ID: {}, affected {} rows", p_id, rows),
            Err(e) => error!("Failed to delete person with ID {}: {}", p_id, e),
        }

        result
    }

    pub fn update_google_id(
        conn: &mut DbConnection,
        p_id: &str,
        g_id: &str,
    ) -> QueryResult<models::Person> {
        use crate::schema::person::dsl::*;
        debug!("Updating Google ID for person with ID: {}", p_id);

        match conn {
            DbConnection::Sqlite(conn) => {
                diesel::update(person.filter(id.eq(p_id)))
                    .set(google_id.eq(g_id))
                    .execute(conn)?;
            }
            DbConnection::Pg(conn) => {
                diesel::update(person.filter(id.eq(p_id)))
                    .set(google_id.eq(g_id))
                    .execute(conn)?;
            }
        }

        Self::get_by_id(conn, p_id)
    }
}
