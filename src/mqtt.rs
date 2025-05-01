use std::{sync::Arc, time::Duration};

use rocket::State;
use rumqttc::{AsyncClient, MqttOptions};

use crate::{conf::Conf, AppState};


pub struct DeviceInfo {
    topics: Vec<String>
}

async fn mqtt_listener(state: Arc<AppState>, config: &State<Conf>) {
    let mut mqttoptions = MqttOptions::new(config.broker_id.clone(), config.broker_url.clone(), config.broker_port);
	mqttoptions.set_keep_alive(Duration::from_millis(config.keep_alive));
	let (mut client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

	// Subscribe to all devices
	client.subscribe("devices/+/register", rumqttc::QoS::AtLeastOnce).await.unwrap(); //  FIXME Remove unwrap	
}
