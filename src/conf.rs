use rocket::figment::{self, Figment};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Conf {
    pub broker_url: String,
    pub broker_port: u16,
    pub broker_id: String,
	pub keep_alive: u64,
	pub session_timeout: u64,
	pub db_str: String,
}


pub fn load_config(figment: &Figment) -> figment::Result<Conf> {
	figment.extract()
}
