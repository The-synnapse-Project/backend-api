mod models;
mod req_logger;
mod routes;
use models::Database;
use req_logger::ReqLogger;
use rocket::{Request, catch, catchers, http::Status};
use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{SwaggerUIConfig, make_swagger_ui},
};
use routes::{entries::*, permissions::*, person::*};

#[catch(404)]
fn not_found(req: &Request) -> String {
    println!("Not found: {} {}", req.method(), req.uri());
    format!(
        "The requested URL {} was not found on this server.",
        req.uri()
    )
}

#[catch(default)]
fn default_catcher(status: Status, req: &Request) -> String {
    println!("Default catcher: {} {} {}", status, req.method(), req.uri());
    format!(
        "An error occurred: {} {} {}",
        status,
        req.method(),
        req.uri()
    )
}

pub async fn run_server(db_url: &str) -> Result<(), rocket::Error> {
    let app_state = Database {
        db_url: db_url.to_string(),
    };
    let build = rocket::build()
        .manage(app_state)
        .attach(ReqLogger {})
        .register("/", catchers![not_found, default_catcher])
        .mount(
            "/",
            openapi_get_routes![
                // Entries
                create_entry,
                get_entries,
                get_entry,
                get_entry_by_person_id,
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
                delete_person
            ],
        )
        .mount(
            "/doc",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_string(),

                ..Default::default()
            }),
        );

    let _ignite = build.launch().await?;
    Ok(())
}
