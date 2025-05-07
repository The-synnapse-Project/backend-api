use db::establish_connection;
use db::interactions::PersonInteractor;
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
    let persons = PersonInteractor::get(conn).unwrap();
    RawJson(serde_json::to_string(&persons).unwrap())
}

/// Get a single person by ID
#[openapi(tag = "Persons")]
#[get("/api/person/<person_id>")]
pub async fn get_person_by_id(db: &State<Database>, person_id: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let person = PersonInteractor::get_by_id(conn, &person_id).unwrap();
    RawJson(serde_json::to_string(&person).unwrap())
}

/// Get a single person by ID
#[openapi(tag = "Persons")]
#[post("/api/person", format = "json", data = "<person>")]
pub async fn create_person(db: &State<Database>, person: Json<Person>) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let new_person = PersonInteractor::new(conn, &person).unwrap();
    RawJson(serde_json::to_string(&new_person).unwrap())
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
    let updated_person = PersonInteractor::update(conn, &person_id, &person).unwrap();
    RawJson(serde_json::to_string(&updated_person).unwrap())
}

/// Delete a person
#[openapi(tag = "Persons")]
#[delete("/api/person/<person_id>")]
pub async fn delete_person(db: &State<Database>, person_id: String) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let deleted_person = PersonInteractor::delete(conn, &person_id).unwrap();
    RawJson(serde_json::to_string(&deleted_person).unwrap())
}
