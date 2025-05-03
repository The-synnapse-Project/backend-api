use std::collections::HashMap;

pub mod mqtt;
pub mod conf;
pub mod db;

use conf::load_config;
use db::{run_migration, setup_db};
use mqtt::DeviceInfo;
use rocket::futures::lock::Mutex;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Default)]
pub struct AppState {
    devices: Mutex<HashMap<String, DeviceInfo>>,
}

#[main]
async fn main() -> Result<(), rocket::Error> {
    let rocket = rocket::build().mount("/", routes![index]);

    let conf = load_config(rocket.figment()).expect("Unable to load config");
	let db = setup_db(conf.db_str, "bridge".to_string()).await.expect("Unable to connect to db");

	run_migration(&db).await.expect("Migrations failed");

	rocket.manage(db).launch().await?;

    Ok(())
}
