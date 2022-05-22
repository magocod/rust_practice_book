use actix_web::http::header::ToStrError;
use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use jsonwebtoken::errors as tk_errors;
use jsonwebtoken::errors::ErrorKind;
use std::convert::From;

use mongodb;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "NotFound: {}", _0)]
    NotFound(String),

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
            ServiceError::NotFound(ref message) => HttpResponse::NotFound().json(message),
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
            ErrorKind::InvalidToken => ServiceError::BadRequest("invalid tk error".into()),
            _ => ServiceError::BadRequest("tk general error".into()),
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

impl From<mongodb::error::Error> for ServiceError {
    fn from(v: mongodb::error::Error) -> ServiceError {
        println!("{:?}", v);
        ServiceError::BadRequest("mongodb Error".into())
    }
}

impl From<mongodb::error::ErrorKind> for ServiceError {
    fn from(v: mongodb::error::ErrorKind) -> ServiceError {
        println!("{:?}", v);
        match v {
            mongodb::error::ErrorKind::InvalidArgument { .. } => {}
            mongodb::error::ErrorKind::Authentication { .. } => {}
            mongodb::error::ErrorKind::BsonDeserialization(_) => {}
            mongodb::error::ErrorKind::BsonSerialization(_) => {}
            mongodb::error::ErrorKind::BulkWrite(_) => {}
            mongodb::error::ErrorKind::Command(_) => {}
            mongodb::error::ErrorKind::DnsResolve { .. } => {}
            mongodb::error::ErrorKind::Internal { .. } => {}
            mongodb::error::ErrorKind::Io(_) => {}
            mongodb::error::ErrorKind::ConnectionPoolCleared { .. } => {}
            mongodb::error::ErrorKind::InvalidResponse { .. } => {}
            mongodb::error::ErrorKind::ServerSelection { .. } => {}
            mongodb::error::ErrorKind::SessionsNotSupported => {}
            mongodb::error::ErrorKind::InvalidTlsConfig { .. } => {}
            mongodb::error::ErrorKind::Write(_) => {}
            mongodb::error::ErrorKind::Transaction { .. } => {}
            mongodb::error::ErrorKind::IncompatibleServer { .. } => {}
            _ => {}
        }
        ServiceError::BadRequest("mongodb ErrorKind".into())
    }
}
