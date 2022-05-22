use crate::errors::ServiceError::BadRequest;
use actix_web::{http::header, HttpRequest, HttpResponse};
use commons::jwt::decode_token;

use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;
use argon2::{self, Config};

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String = "secret".to_string();
}

const SALT: &[u8] = b"supersecuresalt";

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

// WARNING THIS IS ONLY FOR DEMO PLEASE DO MORE RESEARCH FOR PRODUCTION USE
pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let config = Config {
        secret: SECRET_KEY.as_bytes(),
        ..Default::default()
    };
    argon2::hash_encoded(password.as_bytes(), SALT, &config).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    argon2::verify_encoded_ext(hash, password.as_bytes(), SECRET_KEY.as_bytes(), &[]).map_err(
        |err| {
            dbg!(err);
            ServiceError::Unauthorized
        },
    )
}

pub async fn get_me(request: HttpRequest) -> Result<HttpResponse, crate::errors::ServiceError> {
    // println!("{:?}", request.headers());
    let h = request.headers().get(header::AUTHORIZATION);
    // println!("{:?}", h);

    let (_, user) = match h {
        None => return Err(BadRequest("error check token header".into())),
        Some(v) => {
            let tk = v.to_str()?.to_string();
            let mut split = tk.split(" ").collect::<Vec<_>>();
            if split.len() != 2 {
                return Err(BadRequest("error incomplete token header".into()));
            }
            match split.pop() {
                None => return Err(BadRequest("error invalid auth header".into())),
                Some(s) => decode_token(&s.to_string())?,
            }
        }
    };
    // println!("u: {:?}", user);

    Ok(HttpResponse::Ok().json(user))
}
