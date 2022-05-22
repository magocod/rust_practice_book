use serde::{Deserialize, Serialize};

use jsonwebtoken::errors::Result;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};

use crate::mongo::User;

pub const SECRET: &str = "secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub fn generate_token(user: &User) -> Result<String> {
    let my_claims = Claims {
        sub: serde_json::to_string(user)?,
        company: "ACME".to_owned(),
        exp: 10000000000,
    };

    let header = Header {
        kid: Some("signing_key".to_owned()),
        ..Default::default()
    };

    let token = encode(
        &header,
        &my_claims,
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )?;
    // println!("{:?}", token);

    Ok(token)
}

pub fn decode_token(token: &String) -> Result<(TokenData<Claims>, User)> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    // println!("{:?}", token_data.claims.sub);
    // println!("{:?}", token_data.claims);
    // println!("{:?}", token_data.header);

    let user: User = serde_json::from_str(token_data.claims.sub.as_str())?;
    Ok((token_data, user))
}
