use actix_web::http::header::ToStrError;
use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use jsonwebtoken::errors as tk_errors;
use jsonwebtoken::errors::ErrorKind;
use std::convert::From;
use serde_json::error::{Error as SerdeJsonError};
use lapin::{Error as LapinError};
use reqwest::{Error as ReqWestError};

use mongodb;

use mongodb::bson::oid::Error as OidError;

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

impl From<OidError> for ServiceError {
    fn from(v: OidError) -> ServiceError {
        match v {
            OidError::InvalidHexStringCharacter { .. } => {
                ServiceError::BadRequest("mongodb oid Error::InvalidHexStringCharacter".into())
            }
            OidError::InvalidHexStringLength { .. } => {
                ServiceError::BadRequest("mongodb oid Error::InvalidHexStringLength".into())
            }
            _ => {
                ServiceError::BadRequest("mongodb generic oid Error::InvalidHexStringLength".into())
            }
        }
    }
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
// impl ResponseError for mongodb::bson::oid::Error {
//     fn error_response(&self) -> HttpResponse {
//         match self {
//             Error::InvalidHexStringCharacter { .. } => {
//                 HttpResponse::BadRequest().json("mongodb::bson::oid::Error".into())
//             }
//             Error::InvalidHexStringLength { .. } => {
//                 HttpResponse::BadRequest().json("mongodb::bson::oid::Error".into())
//             }
//             _ => {
//                 HttpResponse::BadRequest().json("mongodb::bson::oid::Error general".into())
//             }
//         }
//     }
// }

impl From<SerdeJsonError> for ServiceError {
    fn from(v: SerdeJsonError) -> Self {
        // println!("{}", v);
        ServiceError::BadRequest(v.to_string())
    }
}

impl From<LapinError> for ServiceError {
    fn from(v: LapinError) -> Self {
        // println!("{}", v);
        // match v {
        //     Error::ChannelsLimitReached => {}
        //     Error::InvalidProtocolVersion(_) => {}
        //     Error::InvalidChannel(_) => {}
        //     Error::InvalidChannelState(_) => {}
        //     Error::InvalidConnectionState(_) => {}
        //     Error::IOError(_) => {}
        //     Error::ParsingError(_) => {}
        //     Error::ProtocolError(_) => {}
        //     Error::SerialisationError(_) => {}
        //     _ => {}
        // }
        ServiceError::BadRequest(v.to_string())
    }
}

impl From<ReqWestError> for ServiceError {
    fn from(v: ReqWestError) -> Self {
        // println!("reqwest -> {}", v);
        // println!("source -> {:#?}", v);
        // println!("is_connect -> {}", v.is_connect());
        // println!("is_request -> {}", v.is_request());
        // if v.is_timeout() {
        //     return ServiceError::BadRequest("timeout".to_string());
        // }
        ServiceError::BadRequest(v.to_string())
    }
}