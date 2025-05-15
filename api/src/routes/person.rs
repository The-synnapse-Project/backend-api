use db::establish_connection;
use db::interactions::person::PersonInteractor;
use db::models::Person;
use rocket::serde::json::Json;
use rocket::{State, response::content::RawJson};
use rocket::{delete, get, post, put};
use rocket_okapi::openapi;

use crate::models::Database;

/// Get all persons
#[openapi(tag = "Persons")]
#[get("/api/person")]
pub async fn get_persons(db: &State<Database>) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match PersonInteractor::get(conn) {
        Ok(persons) => RawJson(serde_json::to_string(&persons).unwrap()),
        Err(_) => RawJson(
            "{\"status\": \"error\", \"message\": \"Failed to retrieve persons\"}".to_string(),
        ),
    }
}

/// Get a single person by ID
#[openapi(tag = "Persons")]
#[get("/api/person/<person_id>")]
pub async fn get_person_by_id(db: &State<Database>, person_id: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match PersonInteractor::get_by_id(conn, &person_id) {
        Ok(person) => RawJson(serde_json::to_string(&person).unwrap()),
        Err(_) => RawJson("{\"status\": \"error\", \"message\": \"Person not found\"}".to_string()),
    }
}

/// Create a new person
#[openapi(tag = "Persons")]
#[post("/api/person", format = "json", data = "<person>")]
pub async fn create_person(db: &State<Database>, person: Json<Person>) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match PersonInteractor::new(conn, &person) {
        Ok(new_person) => RawJson(serde_json::to_string(&new_person).unwrap()),
        Err(e) => RawJson(format!(
            "{{\"status\": \"error\", \"message\": \"Failed to create person: {}\"}}",
            e
        )),
    }
}

/// Update an existing person
#[openapi(tag = "Persons")]
#[put("/api/person/<person_id>", format = "json", data = "<person>")]
pub async fn update_person(
    db: &State<Database>,
    person_id: String,
    person: Json<Person>,
) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match PersonInteractor::update(conn, &person_id, &person) {
        Ok(updated_person) => RawJson(serde_json::to_string(&updated_person).unwrap()),
        Err(_) => RawJson(
            "{\"status\": \"error\", \"message\": \"Person not found or update failed\"}"
                .to_string(),
        ),
    }
}

/// Delete a person
#[openapi(tag = "Persons")]
#[delete("/api/person/<person_id>")]
pub async fn delete_person(db: &State<Database>, person_id: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    match PersonInteractor::delete(conn, &person_id) {
        Ok(deleted_person) => RawJson(serde_json::to_string(&deleted_person).unwrap()),
        Err(_) => RawJson(
            "{\"status\": \"error\", \"message\": \"Person not found or delete failed\"}"
                .to_string(),
        ),
    }
}
