use actix_web::{http::header, web, HttpRequest, HttpResponse};
use commons::jwt::decode_token;
use commons::mongo::{User, COLL_USERS, DB_NAME};
use serde::{Deserialize, Serialize};

use crate::errors::ServiceError;
use crate::state::AppState;

use argon2::{self, Config};
use mongodb::bson::doc;

lazy_static::lazy_static! {
    pub static ref SECRET_KEY: String = "secret".to_string();
}

const SALT: &[u8] = b"supersecuresalt";

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthPayload {
    pub user: User,
    pub token: String,
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

pub async fn login(
    auth_data: web::Json<AuthData>,
    data: web::Data<AppState>,
) -> Result<HttpResponse, ServiceError> {
    println!("{:?}", auth_data);

    let collection = data
        .mongodb_client
        .database(DB_NAME)
        .collection::<User>(COLL_USERS);
    let d = doc! {
        "email": &auth_data.email
    };
    let found = collection.find_one(d, None).await?;

    match found {
        None => return Err(ServiceError::NotFound("user not found".into())),
        Some(user) => match verify(&user.password, &auth_data.password) {
            Ok(b) => {
                if !b {
                    return Err(ServiceError::BadRequest("invalid credentials".into()));
                }
                let tk = user.generate_token()?;

                match tk {
                    None => {
                        return Err(ServiceError::BadRequest("failed token generation".into()));
                    }
                    Some(t) => {
                        t.save(&data.mongodb_client).await?;
                        Ok(HttpResponse::Ok().json(AuthPayload {
                            user,
                            token: t.token,
                        }))
                    }
                }
            }
            Err(_) => return Err(ServiceError::BadRequest("error checking password".into())),
        },
    }
}

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn get_me(request: HttpRequest) -> Result<HttpResponse, ServiceError> {
    // println!("{:?}", request.headers());
    let h = request.headers().get(header::AUTHORIZATION);
    // println!("{:?}", h);

    let (_, user) = match h {
        None => return Err(ServiceError::BadRequest("error check token header".into())),
        Some(v) => {
            let tk = v.to_str()?.to_string();
            let mut split = tk.split(" ").collect::<Vec<_>>();
            if split.len() != 2 {
                return Err(ServiceError::BadRequest(
                    "error incomplete token header".into(),
                ));
            }
            match split.pop() {
                None => return Err(ServiceError::BadRequest("error invalid auth header".into())),
                Some(s) => decode_token(&s.to_string())?,
            }
        }
    };
    // println!("u: {:?}", user);

    Ok(HttpResponse::Ok().json(user))
}
