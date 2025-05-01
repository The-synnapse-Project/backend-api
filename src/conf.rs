use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Conf {
    version: u8,
    broker_url: String,
    broker_port: u16,
    broker_id: String,
}

impl ::std::default::Default for Conf {
    fn default() -> Self {
        Self {
            version: Default::default(),
            broker_url: Default::default(),
            broker_port: Default::default(),
            broker_id: Default::default(),
        }
    }
}

pub fn load_config() -> Result<Conf, confy::ConfyError> {
    confy::load("synnapse-bridge", None)
}
