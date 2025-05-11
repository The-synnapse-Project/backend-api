use crate::DbConnection;
use crate::models;
use diesel::prelude::*;

pub struct PermissionsInteractor {}

impl PermissionsInteractor {
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

    pub fn get_by_id(conn: &mut DbConnection, p_id: &str) -> QueryResult<models::Permissions> {
        use crate::schema::permissions::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => permissions.filter(id.eq(p_id)).first(conn),
            DbConnection::Pg(conn) => permissions.filter(id.eq(p_id)).first(conn),
        }
    }

    pub fn update(
        conn: &mut DbConnection,
        p_id: &str,
        permissions_changes: &models::Permissions,
    ) -> QueryResult<usize> {
        use crate::schema::permissions::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => diesel::update(permissions.filter(id.eq(p_id)))
                .set(permissions_changes)
                .execute(conn),
            DbConnection::Pg(conn) => diesel::update(permissions.filter(id.eq(p_id)))
                .set(permissions_changes)
                .execute(conn),
        }
    }
    pub fn delete(conn: &mut DbConnection, p_id: &str) -> QueryResult<usize> {
        use crate::schema::permissions::dsl::*;
        match conn {
            DbConnection::Sqlite(conn) => {
                diesel::delete(permissions.filter(id.eq(p_id))).execute(conn)
            }
            DbConnection::Pg(conn) => diesel::delete(permissions.filter(id.eq(p_id))).execute(conn),
        }
    }
}
