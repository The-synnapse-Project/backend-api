mod cors;
mod models;
mod req_logger;
mod routes;

use crate::cors::CORS;
use crate::models::Database;
use crate::routes::{auth::*, entries::*, misc::*, permissions::*, person::*};
use log::{error, info, warn};
use req_logger::ReqLogger;
use rocket::{Request, catch, catchers, http::Status, options};
use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{SwaggerUIConfig, make_swagger_ui},
};

#[catch(404)]
fn not_found(req: &Request) -> String {
    warn!("Not found: {} {}", req.method(), req.uri());
    format!(
        "The requested URL {} was not found on this server.",
        req.uri()
    )
}

#[catch(default)]
fn default_catcher(status: Status, req: &Request) -> String {
    error!("Error: {} {} {}", status, req.method(), req.uri());
    format!(
        "An error occurred: {} {} {}",
        status,
        req.method(),
        req.uri()
    )
}

#[options("/<_..>")]
fn all_options() {}

pub async fn run_server(db_url: &str) -> Result<(), rocket::Error> {
    // Initialize WebSocketManager
    info!("Starting server with database: {}", db_url);

    let app_state = Database {
        db_url: db_url.to_string(),
    };
    let build = rocket::build()
        .manage(app_state)
        .attach(ReqLogger {})
        .attach(CORS {})
        .register("/", catchers![not_found, default_catcher])
        .mount("/", rocket::routes![all_options])
        .mount(
            "/",
            openapi_get_routes![
                // Entries
                create_entry,
                get_entries,
                get_entry,
                get_entry_by_person_id,
                get_entry_by_date_and_person_id,
                get_entry_by_action,
                get_entry_by_action_and_person_id,
                get_entry_by_date,
                update_entry,
                delete_entry,
                // Permissions
                get_permissions,
                get_permissions_by_person_id,
                get_permissions_by_id,
                create_permissions,
                update_permissions,
                delete_permissions,
                // Person
                create_person,
                get_persons,
                get_person_by_id,
                update_person,
                delete_person,
                // Auth
                login,
                register,
                change_password,
                // Misc
                health_check,
            ],
        )
        .mount(
            "/docs",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_string(),

                ..Default::default()
            }),
        );

    info!("Launching Rocket server...");
    let _ignite = build.launch().await?;
    info!("Server shutdown");
    Ok(())
}
