use crate::auth::crypto;
use rocket::{
    http::Status,
    request::{self, FromRequest, Request},
};
use std::{error::Error, fmt};

struct ApiKey {
    key: String,
}

#[derive(Debug)]
struct UnAuthorizedError {
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

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(api_key) = req.headers().get_one("x-syn-api-key") {
            if crypto::verify_api_key(api_key, &req.uri().to_string()) {
                return request::Outcome::Success(ApiKey {
                    key: api_key.to_string(),
                });
            } else {
                return request::Outcome::Error((
                    Status::Unauthorized,
                    UnAuthorizedError::new(&req.uri().to_string()),
                ));
            }
        }
        request::Outcome::Error((
            Status::Unauthorized,
            UnAuthorizedError::new(&req.uri().to_string()),
        ))
    }
}
