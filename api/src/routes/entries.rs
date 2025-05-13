use db::establish_connection;
use db::interactions::entries::EntriesInteractor;
use db::models::Entry;
use rocket::serde::json::Json;
use rocket::{State, response::content::RawJson};
use rocket::{delete, get, post, put};
use rocket_okapi::openapi;

use crate::models::Database;
/// Get all entries
#[openapi(tag = "Entries")]
#[get("/api/entry")]
pub async fn get_entries(db: &State<Database>) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let entries = EntriesInteractor::get(conn).unwrap();
    RawJson(serde_json::to_string(&entries).unwrap())
}

/// Get a single entry by ID
#[openapi(tag = "Entries")]
#[get("/api/entry/<entry_id>")]
pub async fn get_entry(db: &State<Database>, entry_id: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let entry = EntriesInteractor::get_by_id(conn, &entry_id).unwrap();
    RawJson(serde_json::to_string(&entry).unwrap())
}

/// Get a single entry by person ID
#[openapi(tag = "Entries")]
#[get("/api/entry/by-person/<person_id>")]
pub async fn get_entry_by_person_id(db: &State<Database>, person_id: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let entry = EntriesInteractor::get_by_p_id(conn, &person_id).unwrap();
    RawJson(serde_json::to_string(&entry).unwrap())
}

/// Get a single entry by date
#[openapi(tag = "Entries")]
#[get("/api/entry/by-date/<date>")]
pub async fn get_entry_by_date(db: &State<Database>, date: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let entries = EntriesInteractor::get_by_date(conn, &date).unwrap();
    RawJson(serde_json::to_string(&entries).unwrap())
}

/// Get a single entry by date and person ID
#[openapi(tag = "Entries")]
#[get("/api/entry/by-date/<date>/<person_id>")]
pub async fn get_entry_by_date_and_person_id(
    db: &State<Database>,
    date: String,
    person_id: String,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let entries = EntriesInteractor::get_by_date_and_p_id(conn, &date, &person_id).unwrap();
    RawJson(serde_json::to_string(&entries).unwrap())
}

/// Get a single entry by action
#[openapi(tag = "Entries")]
#[get("/api/entry/by-action/<action>")]
pub async fn get_entry_by_action(db: &State<Database>, action: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let entries = EntriesInteractor::get_by_action(conn, &action).unwrap();
    RawJson(serde_json::to_string(&entries).unwrap())
}

/// Get a single entry by action and person ID
#[openapi(tag = "Entries")]
#[get("/api/entry/by-action/<action>/<person_id>")]
pub async fn get_entry_by_action_and_person_id(
    db: &State<Database>,
    action: String,
    person_id: String,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let entries = EntriesInteractor::get_by_action_and_p_id(conn, &action, &person_id).unwrap();
    RawJson(serde_json::to_string(&entries).unwrap())
}

/// Create a new entry
#[openapi(tag = "Entries")]
#[post("/api/entry", format = "json", data = "<entry>")]
pub async fn create_entry(db: &State<Database>, entry: Json<Entry>) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let new_entry = EntriesInteractor::new(conn, &entry).unwrap();
    RawJson(serde_json::to_string(&new_entry).unwrap())
}

/// Update an existing entry
#[openapi(tag = "Entries")]
#[put("/api/entry/<entry_id>", format = "json", data = "<entry>")]
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
#[delete("/api/entry/<entry_id>")]
pub async fn delete_entry(db: &State<Database>, entry_id: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let deleted_entry = EntriesInteractor::delete(conn, &entry_id).unwrap();
    RawJson(serde_json::to_string(&deleted_entry).unwrap())
}
