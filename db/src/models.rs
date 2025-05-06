use crate::interactions::Action;
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

impl Person {
    pub fn new(name: &str, surname: &str, email: &str, password_hash: &str) -> Self {
        let name = name.to_string();
        let surname = surname.to_string();
        let email = email.to_string();
        let password_hash = password_hash.to_string();

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            surname,
            email,
            password_hash,
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::permissions)]
pub struct Permissions {
    pub id: String,
    pub person_id: String,
    pub dashboard: bool,
    pub see_self_history: bool,
    pub see_others_history: bool,
    pub admin_panel: bool,
    pub edit_permissions: bool,
}

impl Permissions {
    pub fn new(
        person_id: &str,
        dashboard: bool,
        see_self_history: bool,
        see_others_history: bool,
        admin_panel: bool,
        edit_permissions: bool,
    ) -> Self {
        let person_id = person_id.to_string();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            person_id,
            dashboard,
            see_self_history,
            see_others_history,
            admin_panel,
            edit_permissions,
        }
    }

    pub fn update(&mut self, db_url: &str) {
        use crate::schema::permissions::dsl::*;
        let conn = &mut crate::establish_connection(db_url);
        match conn {
            crate::DbConnection::Sqlite(conn) => {
                diesel::update(permissions.filter(id.eq(&self.id)))
                    .set((
                        dashboard.eq(self.dashboard),
                        see_self_history.eq(self.see_self_history),
                        see_others_history.eq(self.see_others_history),
                        admin_panel.eq(self.admin_panel),
                        edit_permissions.eq(self.edit_permissions),
                    ))
                    .execute(conn)
                    .expect("Error updating permissions");
            }
            crate::DbConnection::Pg(conn) => {
                diesel::update(permissions.filter(id.eq(&self.id)))
                    .set((
                        dashboard.eq(self.dashboard),
                        see_self_history.eq(self.see_self_history),
                        see_others_history.eq(self.see_others_history),
                        admin_panel.eq(self.admin_panel),
                        edit_permissions.eq(self.edit_permissions),
                    ))
                    .execute(conn)
                    .expect("Error updating permissions");
            }
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::entries)]
pub struct Entry {
    pub id: String,
    pub person_id: String,
    pub instant: chrono::NaiveDateTime,
    pub action: String,
}

impl Entry {
    pub fn new(person_id: &str, action: Action) -> Self {
        let person_id = person_id.to_string();
        let action = match action {
            Action::Enter => "Enter".to_string(),
            Action::Exit => "Exit".to_string(),
        };
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            person_id,
            instant: chrono::Local::now().naive_utc(),
            action,
        }
    }
}
