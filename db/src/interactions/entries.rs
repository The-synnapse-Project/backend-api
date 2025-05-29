use crate::DbConnection;
use crate::date;
use crate::models;
use diesel::prelude::*;
use log::error;

pub struct EntriesInteractor {}

impl EntriesInteractor {
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
    pub fn get_by_id(conn: &mut DbConnection, e_id: &str) -> QueryResult<models::Entry> {
        use crate::schema::entries::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => entries.filter(id.eq(e_id)).first(conn),
            DbConnection::Pg(conn) => entries.filter(id.eq(e_id)).first(conn),
        }
    }
    pub fn update(
        conn: &mut DbConnection,
        e_id: &str,
        entries_changes: &models::Entry,
    ) -> QueryResult<usize> {
        use crate::schema::entries::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => diesel::update(entries.filter(id.eq(e_id)))
                .set(entries_changes)
                .execute(conn),
            DbConnection::Pg(conn) => diesel::update(entries.filter(id.eq(e_id)))
                .set(entries_changes)
                .execute(conn),
        }
    }
    pub fn delete(conn: &mut DbConnection, e_id: &str) -> QueryResult<usize> {
        use crate::schema::entries::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => diesel::delete(entries.filter(id.eq(e_id))).execute(conn),
            DbConnection::Pg(conn) => diesel::delete(entries.filter(id.eq(e_id))).execute(conn),
        }
    }

    pub fn get_by_date(conn: &mut DbConnection, date: &str) -> QueryResult<Vec<models::Entry>> {
        use crate::schema::entries::dsl::*;
        let req_instant = match date::parse_with_time(&format!("{} 23:59:59", date)) {
            Some(i) => i,
            None => {
                error!("Failed to parse date: {}", date);
                return Err(diesel::result::Error::NotFound);
            }
        };
        match conn {
            DbConnection::Sqlite(conn) => entries
                .filter(instant.le(req_instant))
                .select(models::Entry::as_select())
                .load(conn),
            DbConnection::Pg(conn) => entries
                .filter(instant.le(req_instant))
                .select(models::Entry::as_select())
                .load(conn),
        }
    }

    pub fn get_by_date_and_p_id(
        conn: &mut DbConnection,
        date: &str,
        p_id: &str,
    ) -> QueryResult<Vec<models::Entry>> {
        use crate::schema::entries::dsl::*;

        let req_instant = match date::parse_with_time(&format!("{} 23:59:59", date)) {
            Some(i) => i,
            None => {
                error!("Failed to parse date: {}", date);
                return Err(diesel::result::Error::NotFound);
            }
        };
        match conn {
            DbConnection::Sqlite(conn) => entries
                .filter(instant.le(req_instant).and(person_id.eq(p_id)))
                .select(models::Entry::as_select())
                .load(conn),
            DbConnection::Pg(conn) => entries
                .filter(instant.le(req_instant).and(person_id.eq(p_id)))
                .select(models::Entry::as_select())
                .load(conn),
        }
    }

    pub fn get_by_action(
        conn: &mut DbConnection,
        req_action: &str,
    ) -> QueryResult<Vec<models::Entry>> {
        use crate::schema::entries::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => entries
                .filter(action.eq(req_action))
                .select(models::Entry::as_select())
                .load(conn),
            DbConnection::Pg(conn) => entries
                .filter(action.eq(req_action))
                .select(models::Entry::as_select())
                .load(conn),
        }
    }

    pub fn get_by_action_and_p_id(
        conn: &mut DbConnection,
        req_action: &str,
        p_id: &str,
    ) -> QueryResult<Vec<models::Entry>> {
        use crate::schema::entries::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => entries
                .filter(action.eq(req_action).and(person_id.eq(p_id)))
                .select(models::Entry::as_select())
                .load(conn),
            DbConnection::Pg(conn) => entries
                .filter(action.eq(req_action).and(person_id.eq(p_id)))
                .select(models::Entry::as_select())
                .load(conn),
        }
    }
}

pub enum Action {
    Enter,
    Exit,
}
