use rocket::{Data, Orbit, Request, Response, Rocket, fairing::Fairing};

pub struct ReqLogger {}

#[rocket::async_trait]
impl Fairing for ReqLogger {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Request Logger",
            kind: rocket::fairing::Kind::Request,
        }
    }

    async fn on_liftoff(&self, _rocket: &Rocket<Orbit>) {
        println!("Rocket api running");
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        println!(
            "{} {} {}",
            req.method(),
            req.uri(),
            if let Some(ip) = req.client_ip() {
                ip.to_string()
            } else {
                "unknown".to_string()
            }
        );
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, _res: &mut Response<'r>) {
        println!("Response: {} {}", _req.method(), _req.uri());
    }

    async fn on_shutdown(&self, _rocket: &Rocket<Orbit>) {
        println!("Shutting down");
    }
}
