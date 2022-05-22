use actix_web::http::header::ToStrError;
use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use std::convert::From;
use jsonwebtoken::errors as tk_errors;
use jsonwebtoken::errors::ErrorKind;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}

// impl From<Example> for ServiceError {
//     fn from(_: Example) -> ServiceError {
//         ServiceError::BadRequest("error".into())
//     }
// }

impl From<ToStrError> for ServiceError {
    fn from(_: ToStrError) -> ServiceError {
        ServiceError::BadRequest("error ToStrError".into())
    }
}

impl From<tk_errors::Error> for ServiceError {
    fn from(v: tk_errors::Error) -> ServiceError {
        // println!("e: {:?}", v);
        match v.kind() {
            ErrorKind::InvalidToken => {
                ServiceError::BadRequest("invalid tk error".into())
            }
            _ => { ServiceError::BadRequest("tk general error".into()) }
            // ErrorKind::InvalidSignature => {}
            // ErrorKind::InvalidEcdsaKey => {}
            // ErrorKind::InvalidRsaKey(_) => {}
            // ErrorKind::RsaFailedSigning => {}
            // ErrorKind::InvalidAlgorithmName => {}
            // ErrorKind::InvalidKeyFormat => {}
            // ErrorKind::MissingRequiredClaim(_) => {}
            // ErrorKind::ExpiredSignature => {}
            // ErrorKind::InvalidIssuer => {}
            // ErrorKind::InvalidAudience => {}
            // ErrorKind::InvalidSubject => {}
            // ErrorKind::ImmatureSignature => {}
            // ErrorKind::InvalidAlgorithm => {}
            // ErrorKind::MissingAlgorithm => {}
            // ErrorKind::Base64(_) => {}
            // ErrorKind::Json(_) => {}
            // ErrorKind::Utf8(_) => {}
            // ErrorKind::Crypto(_) => {}
        }
    }
}
