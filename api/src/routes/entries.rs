use db::establish_connection;
use db::interactions::entries::{Action, EntriesInteractor};
use db::models::Entry;
use rocket::serde::json::Json;
use rocket::{State, response::content::RawJson};
use rocket::{delete, get, post, put};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::auth::guard::ApiKey;
use crate::models::Database;

/// Get all entries
#[openapi(tag = "Entries")]
#[get("/api/entry")]
pub async fn get_entries(db: &State<Database>, _api_key: ApiKey) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match EntriesInteractor::get(conn) {
        Ok(entries) => RawJson(serde_json::to_string(&entries).unwrap()),
        Err(_) => RawJson(
            "{\"status\": \"error\", \"message\": \"Failed to retrieve entries\"}".to_string(),
        ),
    }
}

/// Get a single entry by ID
#[openapi(tag = "Entries")]
#[get("/api/entry/<entry_id>")]
pub async fn get_entry(
    db: &State<Database>,
    entry_id: String,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match EntriesInteractor::get_by_id(conn, &entry_id) {
        Ok(entry) => RawJson(serde_json::to_string(&entry).unwrap()),
        Err(_) => RawJson("{\"status\": \"error\", \"message\": \"Entry not found\"}".to_string()),
    }
}

/// Get a single entry by person ID
#[openapi(tag = "Entries")]
#[get("/api/entry/by-person/<person_id>")]
pub async fn get_entry_by_person_id(
    db: &State<Database>,
    person_id: String,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match EntriesInteractor::get_by_p_id(conn, &person_id) {
        Ok(entry) => RawJson(serde_json::to_string(&entry).unwrap()),
        Err(_) => RawJson(
            "{\"status\": \"error\", \"message\": \"No entries found for this person\"}"
                .to_string(),
        ),
    }
}

/// Get a single entry by date
#[openapi(tag = "Entries")]
#[get("/api/entry/by-date/<date>")]
pub async fn get_entry_by_date(
    db: &State<Database>,
    date: String,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match EntriesInteractor::get_by_date(conn, &date) {
        Ok(entries) => RawJson(serde_json::to_string(&entries).unwrap()),
        Err(_) => RawJson(
            "{\"status\": \"error\", \"message\": \"No entries found for this date\"}".to_string(),
        ),
    }
}

/// Get a single entry by date and person ID
#[openapi(tag = "Entries")]
#[get("/api/entry/by-date/<date>/<person_id>")]
pub async fn get_entry_by_date_and_person_id(
    db: &State<Database>,
    date: String,
    person_id: String,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match EntriesInteractor::get_by_date_and_p_id(conn, &date, &person_id) {
        Ok(entries) => RawJson(serde_json::to_string(&entries).unwrap()),
        Err(_) => RawJson(
            "{\"status\": \"error\", \"message\": \"No entries found for this date and person\"}"
                .to_string(),
        ),
    }
}

/// Get a single entry by action
#[openapi(tag = "Entries")]
#[get("/api/entry/by-action/<action>")]
pub async fn get_entry_by_action(
    db: &State<Database>,
    action: String,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match EntriesInteractor::get_by_action(conn, &action) {
        Ok(entries) => RawJson(serde_json::to_string(&entries).unwrap()),
        Err(_) => RawJson(
            "{\"status\": \"error\", \"message\": \"No entries found for this action\"}"
                .to_string(),
        ),
    }
}

/// Get a single entry by action and person ID
#[openapi(tag = "Entries")]
#[get("/api/entry/by-action/<action>/<person_id>")]
pub async fn get_entry_by_action_and_person_id(
    db: &State<Database>,
    action: String,
    person_id: String,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match EntriesInteractor::get_by_action_and_p_id(conn, &action, &person_id) {
        Ok(entries) => RawJson(serde_json::to_string(&entries).unwrap()),
        Err(_) => RawJson(
            "{\"status\": \"error\", \"message\": \"No entries found for this action and person\"}"
                .to_string(),
        ),
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct APIEntry {
    person_id: String,
    action: String,
}

/// Create a new entry
#[openapi(tag = "Entries")]
#[post("/api/entry", format = "json", data = "<entry>")]
pub async fn create_entry(
    db: &State<Database>,
    entry: Json<APIEntry>,
    _api_key: ApiKey,
) -> RawJson<String> {
    let action = match entry.action.to_lowercase().as_str() {
        "entrada" | "enter" => Action::Enter,
        "salida" | "exit" => Action::Exit,
        _ => {
            return RawJson(
                "{\"status\": \"error\", \"message\": \"Invalid  Action\"}".to_string(),
            );
        }
    };
    let entry = Entry::new(&entry.person_id, action);
    let conn = &mut establish_connection(&db.db_url);
    match EntriesInteractor::new(conn, &entry) {
        Ok(new_entry) => RawJson(serde_json::to_string(&new_entry).unwrap()),
        Err(e) => RawJson(format!(
            "{{\"status\": \"error\", \"message\": \"Failed to create entry: {}\"}}",
            e
        )),
    }
}

/// Update an existing entry
#[openapi(tag = "Entries")]
#[put("/api/entry/<entry_id>", format = "json", data = "<entry>")]
pub async fn update_entry(
    db: &State<Database>,
    entry_id: String,
    entry: Json<Entry>,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match EntriesInteractor::update(conn, &entry_id, &entry) {
        Ok(updated_entry) => RawJson(serde_json::to_string(&updated_entry).unwrap()),
        Err(_) => RawJson(
            "{\"status\": \"error\", \"message\": \"Entry not found or update failed\"}"
                .to_string(),
        ),
    }
}

/// Delete an entry
#[openapi(tag = "Entries")]
#[delete("/api/entry/<entry_id>")]
pub async fn delete_entry(
    db: &State<Database>,
    entry_id: String,
    _api_key: ApiKey,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match EntriesInteractor::delete(conn, &entry_id) {
        Ok(deleted_entry) => RawJson(serde_json::to_string(&deleted_entry).unwrap()),
        Err(_) => RawJson(
            "{\"status\": \"error\", \"message\": \"Entry not found or delete failed\"}"
                .to_string(),
        ),
    }
}
