use crate::models::Database;
use rocket::response::content::RawJson;
use rocket::serde::json::Json;
use rocket::{State, post};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[openapi(tag = "Authentication")]
#[post("/api/auth/login", format = "json", data = "<login>")]
pub async fn login(db: &State<Database>, login: Json<Login>) -> RawJson<String> {
    let conn = &mut db::establish_connection(&db.db_url);
    if let Ok(person) = db::interactions::person::PersonInteractor::get_by_email(conn, &login.email)
    {
        if db::crypto::check_hash(&login.password, &person.password_hash) {
            return RawJson("{\"status\":\"ok\"}".into());
        }
        return RawJson("{\"status\":\"Invalid Password\"}".into());
    }
    RawJson("{\"status\":\"Invalid Email\"}".into())
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Register {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
}

#[openapi(tag = "Authentication")]
#[post("/api/auth/register", format = "json", data = "<register>")]
pub async fn register(db: &State<Database>, register: Json<Register>) -> RawJson<String> {
    let conn = &mut db::establish_connection(&db.db_url);

    // Check if user with this email already exists
    if db::interactions::person::PersonInteractor::get_by_email(conn, &register.email).is_ok() {
        return RawJson("{\"status\":\"error\",\"message\":\"Email already registered\"}".into());
    }

    // Create the new person
    let person = db::models::Person::new(
        &register.name,
        &register.surname,
        &register.email,
        db::models::Role::Alumno,
        &db::crypto::to_hash(&register.password),
    );

    // Insert the new person
    if let Err(e) = db::interactions::person::PersonInteractor::new(conn, &person) {
        return RawJson(format!(
            "{{\"status\":\"error\",\"message\":\"Failed to create user: {e}\"}}"
        ));
    }

    let permissions = db::models::Permissions::new(&person.id, true, false, true, false, false);

    if let Err(e) = db::interactions::permissions::PermissionsInteractor::new(conn, &permissions) {
        return RawJson(format!(
            "{{\"status\":\"error\",\"message\":\"Failed to create permissions: {e}\"}}"
        ));
    }

    RawJson("{\"status\":\"ok\",\"message\":\"User registered successfully\"}".into())
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ChangePassword {
    pub email: String,
    pub old_password: String,
    pub new_password: String,
}

#[openapi(tag = "Authentication")]
#[post("/api/auth/change-password", format = "json", data = "<change_pw>")]
pub async fn change_password(
    db: &State<Database>,
    change_pw: Json<ChangePassword>,
) -> RawJson<String> {
    let conn = &mut db::establish_connection(&db.db_url);

    // Check if the user exists
    if let Ok(mut person) =
        db::interactions::person::PersonInteractor::get_by_email(conn, &change_pw.email)
    {
        // Verify old password
        if !db::crypto::check_hash(&change_pw.old_password, &person.password_hash) {
            return RawJson(
                "{\"status\":\"error\",\"message\":\"Current password is incorrect\"}".into(),
            );
        }

        // Update with new password
        person.password_hash = db::crypto::to_hash(&change_pw.new_password);
        if db::interactions::person::PersonInteractor::update(conn, &person.id, &person).is_ok() {
            return RawJson(
                "{\"status\":\"ok\",\"message\":\"Password changed successfully\"}".into(),
            );
        } else {
            return RawJson(
                "{\"status\":\"error\",\"message\":\"Failed to update password\"}".into(),
            );
        }
    }

    RawJson("{\"status\":\"error\",\"message\":\"User not found\"}".into())
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PasswordResetRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PasswordResetVerify {
    pub token: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PasswordReset {
    pub token: String,
    pub new_password: String,
}
