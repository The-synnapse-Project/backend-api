use std::collections::HashMap;

pub mod mqtt;
pub mod conf;

use conf::load_config;
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
    let rocket = rocket::build().mount("/", routes![index]).launch().await?;

    println!("{:?}", load_config(rocket.figment()).unwrap());

    Ok(())
}
