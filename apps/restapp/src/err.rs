use actix_web::{
    error, get,
    http::{header, StatusCode},
    HttpResponse,
};
use std::fmt::{Display, Formatter};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorData {
    message: String,
    error: String,
    // code: MyError,
}

impl Display for ErrorData {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}, {}", self.message, self.error)
    }
}

impl error::ResponseError for ErrorData {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        let ed = ErrorData {
            message: self.message.clone(),
            error: self.error.clone(),
        };
        HttpResponse::build(self.status_code())
            .insert_header(header::ContentType::json())
            .json(ed)
    }
}

#[derive(Debug)]
pub enum MyError {
    InternalError,
    BadClientData,
    Timeout,
}

impl Display for MyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::InternalError => {
                write!(f, "internal error")
            }
            MyError::BadClientData => {
                write!(f, "bad request")
            }
            MyError::Timeout => {
                write!(f, "timeout")
            }
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(header::ContentType::html())
            .body(self.to_string())
    }
}

#[get("/custom_error")]
pub async fn custom_error() -> Result<&'static str, MyError> {
    Err(MyError::BadClientData)
}

#[get("/custom_error_json")]
pub async fn custom_error_json() -> Result<&'static str, ErrorData> {
    Err(ErrorData { message: String::from("..."), error: String::from("bad") })
}
