mod models;
mod routes;
use models::Database;
use rocket_okapi::{
    openapi_get_routes,
    settings::UrlObject,
    swagger_ui::{SwaggerUIConfig, make_swagger_ui},
};
use routes::{entries::*, permissions::*, person::*};

pub async fn run_server(db_url: &str) -> Result<(), rocket::Error> {
    let app_state = Database {
        db_url: db_url.to_string(),
    };
    let build = rocket::build()
        .manage(app_state)
        .mount(
            "/api/entries",
            openapi_get_routes![
                create_entry,
                get_entries,
                get_entry,
                get_entry_by_person_id,
                update_entry,
                delete_entry
            ],
        )
        .mount(
            "/api/permissions",
            openapi_get_routes![
                get_permissions,
                get_permissions_by_person_id,
                get_permissions_by_id,
                create_permissions,
                update_permissions,
                delete_permissions
            ],
        )
        .mount(
            "/api/person",
            openapi_get_routes![
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
                urls: vec![
                    UrlObject::new("Person", "/api/person/openapi.json"),
                    UrlObject::new("Entries", "/api/entries/openapi.json"),
                    UrlObject::new("Permissions", "/api/permissions/openapi.json"),
                ],

                ..Default::default()
            }),
        );

    let _ignite = build.launch().await?;
    Ok(())
}
