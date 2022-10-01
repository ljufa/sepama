use std::fmt::Display;

use actix_web::{error::ResponseError, HttpResponse};

#[derive(Debug)]
pub enum ServiceError {
    InternalServerError,
    BadRequest(String),
    JWKSFetchError,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::JWKSFetchError => {
                HttpResponse::InternalServerError().json("Could not fetch JWKS")
            }
        }
    }
}
impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.status_code())
    }
}
