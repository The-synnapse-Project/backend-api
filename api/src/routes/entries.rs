use db::establish_connection;
use db::interactions::EntriesInteractor;
use db::models::Entry;
use rocket::serde::json::Json;
use rocket::{State, response::content::RawJson};
use rocket::{delete, get, post, put};
use rocket_okapi::openapi;

use crate::models::Database;

/// Get all entries
#[openapi(tag = "Entries")]
#[get("/")]
pub async fn get_entries(db: &State<Database>) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let entries = EntriesInteractor::get(conn).unwrap();
    RawJson(serde_json::to_string(&entries).unwrap())
}

/// Get a single entry by ID
#[openapi(tag = "Entries")]
#[get("/<entry_id>")]
pub async fn get_entry(db: &State<Database>, entry_id: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let entry = EntriesInteractor::get_by_id(conn, &entry_id).unwrap();
    RawJson(serde_json::to_string(&entry).unwrap())
}

/// Get a single entry by person ID
#[openapi(tag = "Entries")]
#[get("/by-person/<person_id>")]
pub async fn get_entry_by_person_id(db: &State<Database>, person_id: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let entry = EntriesInteractor::get_by_p_id(conn, &person_id).unwrap();
    RawJson(serde_json::to_string(&entry).unwrap())
}

/// Create a new entry
#[openapi(tag = "Entries")]
#[post("/", format = "json", data = "<entry>")]
pub async fn create_entry(db: &State<Database>, entry: Json<Entry>) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let new_entry = EntriesInteractor::new(conn, &entry).unwrap();
    RawJson(serde_json::to_string(&new_entry).unwrap())
}

/// Update an existing entry
#[openapi(tag = "Entries")]
#[put("/<entry_id>", format = "json", data = "<entry>")]
pub async fn update_entry(
    db: &State<Database>,
    entry_id: String,
    entry: Json<Entry>,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let updated_entry = EntriesInteractor::update(conn, &entry_id, &entry).unwrap();
    RawJson(serde_json::to_string(&updated_entry).unwrap())
}

/// Delete an entry
#[openapi(tag = "Entries")]
#[delete("/<entry_id>")]
pub async fn delete_entry(db: &State<Database>, entry_id: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let deleted_entry = EntriesInteractor::delete(conn, &entry_id).unwrap();
    RawJson(serde_json::to_string(&deleted_entry).unwrap())
}
