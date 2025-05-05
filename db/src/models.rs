use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::person)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Person {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::permissions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Permissions {
    pub id: String,
    pub dashboard: bool,
    pub see_self_history: bool,
    pub see_others_history: bool,
    pub admin_panel: bool,
    pub edit_permissions: bool,
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Entry {
    pub id: String,
    pub instant: String,
    pub action: String,
}
