use std::{collections::HashMap, time::Instant};

use log::warn;
use once_cell::sync::Lazy;
use rocket::{Data, Request, Response, fairing::Fairing};
use std::sync::Mutex;

pub struct ReqLogger {}

static TIMINGS: Lazy<Mutex<HashMap<String, Instant>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[rocket::async_trait]
impl Fairing for ReqLogger {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Request Logger",
            kind: rocket::fairing::Kind::Request | rocket::fairing::Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        let now = Instant::now();
        let id = format!(
            "{} {} {}",
            req.method(),
            req.uri(),
            if let Some(ip) = req.client_ip() {
                ip.to_string()
            } else {
                "unknown".to_string()
            }
        );
        TIMINGS.lock().unwrap().insert(id.clone(), now);
        warn!(
            "Request: {} {} from {}",
            req.method(),
            req.uri(),
            if let Some(ip) = req.client_ip() {
                ip.to_string()
            } else {
                "unknown".to_string()
            }
        );
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let now = Instant::now();
        let id = format!(
            "{} {} {}",
            req.method(),
            req.uri(),
            if let Some(ip) = req.client_ip() {
                ip.to_string()
            } else {
                "unknown".to_string()
            }
        );
        let mut msg = format!(
            "{} {} {} {}",
            req.method(),
            req.uri(),
            if let Some(ip) = req.client_ip() {
                ip.to_string()
            } else {
                "unknown".to_string()
            },
            res.status()
        );
        if let Ok(mut lock) = TIMINGS.lock() {
            if let Some(start_time) = lock.remove(&id) {
                let duration = now.duration_since(start_time);
                msg.push_str(&format!(" {}ms", duration.as_millis()));
            }
        }
        warn!("Response: {msg}");
    }
}
