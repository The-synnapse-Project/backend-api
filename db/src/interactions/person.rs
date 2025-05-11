use crate::DbConnection;
use crate::models;
use diesel::prelude::*;

pub struct PersonInteractor {}

impl PersonInteractor {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(conn: &mut DbConnection, person: &models::Person) -> QueryResult<usize> {
        use crate::schema::person;
        match conn {
            DbConnection::Sqlite(conn) => diesel::insert_into(person::table)
                .values(person)
                .execute(conn),
            DbConnection::Pg(conn) => diesel::insert_into(person::table)
                .values(person)
                .execute(conn),
        }
    }

    pub fn get(conn: &mut DbConnection) -> QueryResult<Vec<models::Person>> {
        use crate::schema::person::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => person.load::<models::Person>(conn),
            DbConnection::Pg(conn) => person.load::<models::Person>(conn),
        }
    }

    pub fn get_by_id(conn: &mut DbConnection, p_id: &str) -> QueryResult<models::Person> {
        use crate::schema::person::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => person.filter(id.eq(p_id)).first(conn),
            DbConnection::Pg(conn) => person.filter(id.eq(p_id)).first(conn),
        }
    }

    pub fn get_by_email(conn: &mut DbConnection, req_email: &str) -> QueryResult<models::Person> {
        use crate::schema::person::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => person.filter(email.eq(req_email)).first(conn),
            DbConnection::Pg(conn) => person.filter(email.eq(req_email)).first(conn),
        }
    }

    pub fn update(
        conn: &mut DbConnection,
        p_id: &str,
        person_changes: &models::Person,
    ) -> QueryResult<usize> {
        use crate::schema::person::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => diesel::update(person.filter(id.eq(p_id)))
                .set(person_changes)
                .execute(conn),
            DbConnection::Pg(conn) => diesel::update(person.filter(id.eq(p_id)))
                .set(person_changes)
                .execute(conn),
        }
    }

    pub fn delete(conn: &mut DbConnection, p_id: &str) -> QueryResult<usize> {
        use crate::schema::person::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => diesel::delete(person.filter(id.eq(p_id))).execute(conn),
            DbConnection::Pg(conn) => diesel::delete(person.filter(id.eq(p_id))).execute(conn),
        }
    }
}
