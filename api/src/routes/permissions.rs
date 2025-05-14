use crate::models::Database;
use db::establish_connection;
use db::interactions::permissions::PermissionsInteractor;
use db::models::Permissions;
use rocket::serde::json::Json;
use rocket::{State, response::content::RawJson};
use rocket::{delete, get, post, put};
use rocket_okapi::openapi;

/// Get all permissions
#[openapi(tag = "Permissions")]
#[get("/api/permission")]
pub async fn get_permissions(db: &State<Database>) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match PermissionsInteractor::get(conn) {
        Ok(permissions) => RawJson(serde_json::to_string(&permissions).unwrap()),
        Err(_) => RawJson("{\"status\": \"error\", \"message\": \"Failed to retrieve permissions\"}".to_string()),
    }
}

/// Get a single permission by ID
#[openapi(tag = "Permissions")]
#[get("/api/permission/by-person/<person_id>")]
pub async fn get_permissions_by_person_id(
    db: &State<Database>,
    person_id: String,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match PermissionsInteractor::get_by_p_id(conn, &person_id) {
        Ok(permissions) => RawJson(serde_json::to_string(&permissions).unwrap()),
        Err(_) => RawJson("{\"status\": \"error\", \"message\": \"Permissions not found for this person\"}".to_string()),
    }
}

/// Get a single permission by ID
#[openapi(tag = "Permissions")]
#[get("/api/permission/<permission_id>")]
pub async fn get_permissions_by_id(db: &State<Database>, permission_id: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match PermissionsInteractor::get_by_id(conn, &permission_id) {
        Ok(permissions) => RawJson(serde_json::to_string(&permissions).unwrap()),
        Err(_) => RawJson("{\"status\": \"error\", \"message\": \"Permission not found\"}".to_string()),
    }
}
/// Create a new permission
#[openapi(tag = "Permissions")]
#[post("/api/permission", format = "json", data = "<permissions>")]
pub async fn create_permissions(
    db: &State<Database>,
    permissions: Json<Permissions>,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match PermissionsInteractor::new(conn, &permissions) {
        Ok(new_permissions) => RawJson(serde_json::to_string(&new_permissions).unwrap()),
        Err(e) => RawJson(format!("{{\"status\": \"error\", \"message\": \"Failed to create permissions: {}\"}}", e)),
    }
}

/// Update an existing permission
#[openapi(tag = "Permissions")]
#[put(
    "/api/permission/<permission_id>",
    format = "json",
    data = "<permissions>"
)]
pub async fn update_permissions(
    db: &State<Database>,
    permission_id: String,
    permissions: Json<Permissions>,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match PermissionsInteractor::update(conn, &permission_id, &permissions) {
        Ok(updated_permissions) => RawJson(serde_json::to_string(&updated_permissions).unwrap()),
        Err(_) => RawJson("{\"status\": \"error\", \"message\": \"Permission not found or update failed\"}".to_string()),
    }
}

/// Delete a permission
#[openapi(tag = "Permissions")]
#[delete("/api/permission/<permission_id>")]
pub async fn delete_permissions(db: &State<Database>, permission_id: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match PermissionsInteractor::delete(conn, &permission_id) {
        Ok(deleted_permissions) => RawJson(serde_json::to_string(&deleted_permissions).unwrap()),
        Err(_) => RawJson("{\"status\": \"error\", \"message\": \"Permission not found or delete failed\"}".to_string()),
    }
}
