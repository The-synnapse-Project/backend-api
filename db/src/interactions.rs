use crate::DbConnection;
use crate::models;
use diesel::SelectableHelper;
use diesel::prelude::QueryResult;
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{ExpressionMethods, RunQueryDsl};

pub struct Person {}

impl Person {
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
}

pub struct Permissions {}

impl Permissions {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(conn: &mut DbConnection, permissions: &models::Permissions) -> QueryResult<usize> {
        use crate::schema::permissions;
        match conn {
            DbConnection::Sqlite(conn) => diesel::insert_into(permissions::table)
                .values(permissions)
                .execute(conn),
            DbConnection::Pg(conn) => diesel::insert_into(permissions::table)
                .values(permissions)
                .execute(conn),
        }
    }

    pub fn get(conn: &mut DbConnection) -> QueryResult<Vec<models::Permissions>> {
        use crate::schema::permissions::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => permissions
                .select(models::Permissions::as_select())
                .load(conn),
            DbConnection::Pg(conn) => permissions
                .select(models::Permissions::as_select())
                .load(conn),
        }
    }

    pub fn get_by_p_id(
        conn: &mut DbConnection,
        p_id: &str,
    ) -> QueryResult<Vec<models::Permissions>> {
        use crate::schema::permissions::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => permissions
                .filter(person_id.eq(p_id))
                .select(models::Permissions::as_select())
                .load(conn),
            DbConnection::Pg(conn) => permissions
                .filter(person_id.eq(p_id))
                .select(models::Permissions::as_select())
                .load(conn),
        }
    }
}

pub struct Entries {}

impl Entries {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(conn: &mut DbConnection, entries: &models::Entry) -> QueryResult<usize> {
        use crate::schema::entries;
        match conn {
            DbConnection::Sqlite(conn) => diesel::insert_into(entries::table)
                .values(entries)
                .execute(conn),
            DbConnection::Pg(conn) => diesel::insert_into(entries::table)
                .values(entries)
                .execute(conn),
        }
    }

    pub fn get(conn: &mut DbConnection) -> QueryResult<Vec<models::Entry>> {
        use crate::schema::entries::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => entries.select(models::Entry::as_select()).load(conn),
            DbConnection::Pg(conn) => entries.select(models::Entry::as_select()).load(conn),
        }
    }

    pub fn get_by_p_id(conn: &mut DbConnection, p_id: &str) -> QueryResult<Vec<models::Entry>> {
        use crate::schema::entries::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => entries
                .filter(person_id.eq(p_id))
                .select(models::Entry::as_select())
                .load(conn),
            DbConnection::Pg(conn) => entries
                .filter(person_id.eq(p_id))
                .select(models::Entry::as_select())
                .load(conn),
        }
    }
}

pub enum Action {
    Enter,
    Exit,
}
