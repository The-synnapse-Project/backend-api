use crate::auth::guard::ApiKey;
use crate::models::Database;
use rocket::response::content::RawJson;
use rocket::serde::json::Json;
use rocket::{State, post};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GoogleLogin {
    pub google_id: String,
    pub email: String,
}

#[openapi(tag = "Authentication")]
#[post("/api/auth/google-login", format = "json", data = "<login>")]
pub async fn google_login(
    db: &State<Database>,
    login: Json<GoogleLogin>,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut db::establish_connection(&db.db_url);

    // First, try to find the user by Google ID
    if let Ok(person) =
        db::interactions::person::PersonInteractor::get_by_google_id(conn, &login.google_id)
    {
        return RawJson(format!(
            "{{\"status\":\"ok\",\"user\":{{\"id\":\"{}\",\"name\":\"{}\",\"email\":\"{}\",\"role\":\"{}\"}}}}",
            person.id, person.name, person.email, person.role
        ));
    }

    // If not found by Google ID, try by email
    if let Ok(person) = db::interactions::person::PersonInteractor::get_by_email(conn, &login.email)
    {
        if person.google_id.is_none() {
            // User exists but doesn't have Google ID linked
            // In a production system, you might want to update the user with the Google ID
            return RawJson(format!(
                "{{\"status\":\"ok\",\"message\":\"User found by email but not linked to Google ID\",\"user\":{{\"id\":\"{}\",\"name\":\"{}\",\"email\":\"{}\",\"role\":\"{}\"}}}}",
                person.id, person.name, person.email, person.role
            ));
        } else {
            // User has a different Google ID linked
            return RawJson("{\"status\":\"error\",\"message\":\"Email already linked to a different Google account\"}".into());
        }
    }

    // User not found
    RawJson("{\"status\":\"error\",\"message\":\"User not found\"}".into())
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UpdateGoogleId {
    pub person_id: String,
    pub google_id: String,
}

#[openapi(tag = "Authentication")]
#[post("/api/auth/update-google-id", format = "json", data = "<update_req>")]
pub async fn update_google_id(
    db: &State<Database>,
    update_req: Json<UpdateGoogleId>,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut db::establish_connection(&db.db_url);

    // Verify the user exists
    if db::interactions::person::PersonInteractor::get_by_id(conn, &update_req.person_id).is_ok() {
        // Update the Google ID
        match db::interactions::person::PersonInteractor::update_google_id(
            conn,
            &update_req.person_id,
            &update_req.google_id,
        ) {
            Ok(_) => {
                return RawJson(
                    "{\"status\":\"ok\",\"message\":\"Google ID updated successfully\"}".into(),
                );
            }
            Err(e) => {
                return RawJson(format!(
                    "{{\"status\":\"error\",\"message\":\"Failed to update Google ID: {e}\"}}"
                ));
            }
        }
    }

    RawJson("{\"status\":\"error\",\"message\":\"User not found\"}".into())
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct GoogleRegister {
    pub google_id: String,
    pub email: String,
    pub name: String,
    pub surname: String,
}

#[openapi(tag = "Authentication")]
#[post("/api/auth/register-google", format = "json", data = "<login>")]
pub async fn google_register(
    db: &State<Database>,
    login: Json<GoogleRegister>,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut db::establish_connection(&db.db_url);

    // Check if the user already exists
    if let Ok(person) =
        db::interactions::person::PersonInteractor::get_by_google_id(conn, &login.google_id)
    {
        return RawJson(format!(
            "{{\"status\":\"ok\",\"user\":{{\"id\":\"{}\",\"name\":\"{}\",\"email\":\"{}\",\"role\":\"{}\"}}}}",
            person.id, person.name, person.email, person.role
        ));
    }

    let person = db::models::Person::new(
        &login.name,
        &login.surname,
        &login.email,
        db::models::Role::Alumno,
        None,
        Some(&login.google_id),
    );

    // Create a new user
    match db::interactions::person::PersonInteractor::new(conn, &person) {
        Ok(_) => {
            // Create default permissions for the new user
            let permissions =
                db::models::Permissions::new(&person.id, true, true, false, false, false);
            match db::interactions::permissions::PermissionsInteractor::new(conn, &permissions) {
                Ok(_) => {
                    // Return the created user data
                    RawJson(format!(
                        "{{\"status\":\"ok\",\"message\":\"User created successfully\",\"user\":{{\"id\":\"{}\",\"name\":\"{}\",\"email\":\"{}\",\"role\":\"{}\"}}}}",
                        person.id, person.name, person.email, person.role
                    ))
                }
                Err(e) => RawJson(format!(
                    "{{\"status\":\"error\",\"message\":\"Failed to create permissions: {e}\"}}"
                )),
            }
        }
        Err(e) => RawJson(format!(
            "{{\"status\":\"error\",\"message\":\"Failed to create user: {e}\"}}"
        )),
    }
}
