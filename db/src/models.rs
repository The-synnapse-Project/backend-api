use std::fmt::Display;

use crate::interactions::entries::Action;
use diesel::{expression::AsExpression, prelude::*, sql_types::Text};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, AsExpression)]
#[diesel(sql_type = Text)]
pub enum Role {
    Admin,
    Profesor,
    Alumno,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "Admin"),
            Role::Profesor => write!(f, "Profesor"),
            Role::Alumno => write!(f, "Alumno"),
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, AsChangeset, JsonSchema)]
#[diesel(table_name = crate::schema::person)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Person {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub role: String,
    pub password_hash: Option<String>,
    pub google_id: Option<String>,
}

impl Person {
    pub fn new(
        name: &str,
        surname: &str,
        email: &str,
        role: Role,
        password_hash: Option<&str>,
        google_id: Option<&str>,
    ) -> Self {
        let name = name.to_string();
        let surname = surname.to_string();
        let email = email.to_string();
        let password_hash = password_hash.map(|s| s.to_string());
        let google_id = google_id.map(|s| s.to_string());

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            surname,
            email,
            role: role.to_string(),
            password_hash,
            google_id,
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, AsChangeset, JsonSchema)]
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
                        dashboard.eq(&self.dashboard),
                        see_self_history.eq(&self.see_self_history),
                        see_others_history.eq(&self.see_others_history),
                        admin_panel.eq(&self.admin_panel),
                        edit_permissions.eq(&self.edit_permissions),
                    ))
                    .execute(conn)
                    .expect("Error updating permissions");
            }
            crate::DbConnection::Pg(conn) => {
                diesel::update(permissions.filter(id.eq(&self.id)))
                    .set((
                        dashboard.eq(&self.dashboard),
                        see_self_history.eq(&self.see_self_history),
                        see_others_history.eq(&self.see_others_history),
                        admin_panel.eq(&self.admin_panel),
                        edit_permissions.eq(&self.edit_permissions),
                    ))
                    .execute(conn)
                    .expect("Error updating permissions");
            }
        }
    }
}

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, AsChangeset, JsonSchema)]
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

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, AsChangeset, JsonSchema)]
#[diesel(table_name = crate::schema::password_reset_tokens)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PasswordResetToken {
    pub id: String,
    pub email: String,
    pub token: String,
    pub expires_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
}

impl PasswordResetToken {
    pub fn new(email: &str, expires_hours: i64) -> Self {
        let token = Self::generate_token();
        let now = chrono::Utc::now().naive_utc();
        let expires_at = now + chrono::Duration::hours(expires_hours);

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            email: email.to_string(),
            token,
            expires_at,
            created_at: now,
        }
    }

    pub fn generate_token() -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                               abcdefghijklmnopqrstuvwxyz\
                               0123456789";
        const TOKEN_LEN: usize = 32;

        let mut rng = rand::thread_rng();

        (0..TOKEN_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    pub fn is_valid(&self) -> bool {
        let now = chrono::Utc::now().naive_utc();
        self.expires_at > now
    }
}
