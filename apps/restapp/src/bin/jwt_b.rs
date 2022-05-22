use serde::{Deserialize, Serialize};

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
    is_revoked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Content {
    id: u32,
    name: String,
}

fn main() {
    let c = Content {
        id: 1,
        name: "hello".to_string(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&c).expect("not serialize");

    let my_claims = Claims {
        sub: j,
        company: "ACME".to_owned(),
        exp: 10000000000,
        is_revoked: false
    };
    let key = b"secret";

    let header = Header {
        kid: Some("signing_key".to_owned()),
        ..Default::default()
    };

    let token = match encode(&header, &my_claims, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(_) => panic!(), // in practice you would return the error
    };
    println!("{:?}", token);

    let token_data = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(key),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!(), // Example on how to handle a specific error
            _ => panic!(),
        },
    };

    let cp: Content =
        serde_json::from_str(token_data.claims.sub.as_str()).expect("failed token sub parse");

    println!("{:?}", token_data.claims.sub);
    println!("parse: {:?}", cp);
    println!("{:?}", token_data.claims);
    println!("{:?}", token_data.header);
}
