use db::establish_connection;
use db::interactions::person::PersonInteractor;
use rocket::get;
use rocket::{State, response::content::RawJson};
use rocket_okapi::openapi;

use crate::auth::guard::ApiKey;
use crate::models::Database;

/// Test api health
#[openapi(tag = "Health")]
#[get("/health")]
pub async fn health_check(db: &State<Database>, _api_key: ApiKey) -> RawJson<String> {
    let conn = &mut establish_connection(&db.db_url);
    let _ = match PersonInteractor::get(conn) {
        Ok(p) => p,
        Err(e) => {
            return RawJson(format!(
                "{{\"status\": \"ok\", \"db_status\": \"Error: {}\"}}",
                e.to_string().replace("\"", "'")
            ));
        }
    };
    RawJson("{\"status\": \"ok\", \"db_status\": \"ok\"}".to_string())
}
