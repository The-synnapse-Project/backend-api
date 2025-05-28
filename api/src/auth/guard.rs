use crate::auth::crypto;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use rocket_okapi::request::OpenApiFromRequest;
use std::{env, error::Error, fmt};

#[derive(OpenApiFromRequest)]
pub struct ApiKey;

#[derive(Debug)]
pub struct UnAuthorizedError {
    route: String,
}

impl UnAuthorizedError {
    fn new(route: &str) -> Self {
        UnAuthorizedError {
            route: route.to_string(),
        }
    }
}

impl Error for UnAuthorizedError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }

    fn description(&self) -> &str {
        "Unauthorized access attempted"
    }
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for UnAuthorizedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unauthorized access to route: {}", self.route)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = UnAuthorizedError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if env::var("SYN_DISABLE_AUTH").unwrap_or("0".to_string()) == "1" {
            return Outcome::Success(ApiKey);
        }
        if let Some(api_key) = req.headers().get_one("X-Syn-Api-Key") {
            if crypto::verify_api_key(api_key, &req.uri().to_string()) {
                return Outcome::Success(ApiKey);
            } else {
                return Outcome::Error((
                    Status::Unauthorized,
                    UnAuthorizedError::new(&req.uri().to_string()),
                ));
            }
        }
        Outcome::Error((
            Status::Unauthorized,
            UnAuthorizedError::new(&req.uri().to_string()),
        ))
    }
}
