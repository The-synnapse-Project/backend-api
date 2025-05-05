use db::{establish_connection, models::*};
use diesel::prelude::*;
use rocket::{State, response::content::RawJson};

#[macro_use]
extern crate rocket;

#[get("/persons")]
fn persons(db: &State<Database>) -> RawJson<String> {
    use db::schema::person::dsl::*;
    let db_url = &db.db_url;

    let connection = &mut establish_connection(db_url);
    let res = person.select(Person::as_select()).load(connection);

    match res {
        Ok(persons) => RawJson(serde_json::to_string(&persons).unwrap()),
        Err(_e) => RawJson("{\"error\": \"Error loading persons\"}".to_string()),
    }
}

#[get("/entries")]
fn entries(db: &State<Database>) -> &'static str {
    use db::schema::entries::dsl::*;
    let db_url = &db.db_url;
    let connection = &mut establish_connection(db_url);
    let res = entries.select(Entry::as_select()).load(connection);
    match res {
        Ok(g_entries) => {
            let mut result = String::new();
            for e in g_entries {
                result.push_str(&format!("{}: {} <{}>\n", e.id, e.instant, e.action));
            }
            Box::leak(result.into_boxed_str())
        }
        Err(_e) => "Error loading entries",
    }
}
#[get("/permissions")]
fn permissions(db: &State<Database>) -> &'static str {
    use db::schema::permissions::dsl::*;
    let db_url = &db.db_url;
    let connection = &mut establish_connection(db_url);
    let res = permissions
        .select(Permissions::as_select())
        .load(connection);
    match res {
        Ok(g_permissions) => {
            let mut result = String::new();
            for p in g_permissions {
                result.push_str(&format!(
                    "{}: {} {} {} {} {}\n",
                    p.id,
                    p.dashboard,
                    p.see_self_history,
                    p.see_others_history,
                    p.admin_panel,
                    p.edit_permissions
                ));
            }
            Box::leak(result.into_boxed_str())
        }
        Err(_e) => "Error loading permissions",
    }
}

struct Database {
    db_url: String,
}

pub async fn run_server(db_url: &str) -> Result<(), rocket::Error> {
    let app_state = Database {
        db_url: db_url.to_string(),
    };
    let build = rocket::build()
        .manage(app_state)
        .mount("/api", routes![persons, entries, permissions]);

    let _ignite = build.launch().await?;
    Ok(())
}
