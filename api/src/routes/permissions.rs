use crate::models::Database;
use db::establish_connection;
use db::interactions::PermissionsInteractor;
use db::models::Permissions;
use rocket::serde::json::Json;
use rocket::{State, response::content::RawJson};
use rocket::{delete, get, post, put};
use rocket_okapi::openapi;

/// Get all permissions
#[openapi(tag = "Permissions")]
#[get("/")]
pub async fn get_permissions(db: &State<Database>) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let permissions = PermissionsInteractor::get(conn).unwrap();
    RawJson(serde_json::to_string(&permissions).unwrap())
}

/// Get a single permission by ID
#[openapi(tag = "Permissions")]
#[get("/by-person/<person_id>")]
pub async fn get_permissions_by_person_id(
    db: &State<Database>,
    person_id: String,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let permissions = PermissionsInteractor::get_by_p_id(conn, &person_id).unwrap();
    RawJson(serde_json::to_string(&permissions).unwrap())
}

/// Get a single permission by ID
#[openapi(tag = "Permissions")]
#[get("/<permission_id>")]
pub async fn get_permissions_by_id(db: &State<Database>, permission_id: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let permissions = PermissionsInteractor::get_by_id(conn, &permission_id).unwrap();
    RawJson(serde_json::to_string(&permissions).unwrap())
}
/// Create a new permission
#[openapi(tag = "Permissions")]
#[post("/", format = "json", data = "<permissions>")]
pub async fn create_permissions(
    db: &State<Database>,
    permissions: Json<Permissions>,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let new_permissions = PermissionsInteractor::new(conn, &permissions).unwrap();
    RawJson(serde_json::to_string(&new_permissions).unwrap())
}

/// Update an existing permission
#[openapi(tag = "Permissions")]
#[put("/<permission_id>", format = "json", data = "<permissions>")]
pub async fn update_permissions(
    db: &State<Database>,
    permission_id: String,
    permissions: Json<Permissions>,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let updated_permissions =
        PermissionsInteractor::update(conn, &permission_id, &permissions).unwrap();
    RawJson(serde_json::to_string(&updated_permissions).unwrap())
}

/// Delete a permission
#[openapi(tag = "Permissions")]
#[delete("/<permission_id>")]
pub async fn delete_permissions(db: &State<Database>, permission_id: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let deleted_permissions = PermissionsInteractor::delete(conn, &permission_id).unwrap();
    RawJson(serde_json::to_string(&deleted_permissions).unwrap())
}
