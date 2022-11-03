
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm, errors::Result};
use chrono::{Duration, Utc};
use rocket::http::hyper::HeaderValue;
use rocket::http::hyper::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

use super::errors::{Error, Result as JWTResult};

type WebResult<T> = std::result::Result<T, rocket::http::Status>;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64
}

const JWT_SECRET: &[u8] = b"secret";


pub fn create_token(user_id: String) -> JWTResult<String> {
    let expiration = Utc::now().checked_add_signed(Duration::seconds(3600)).expect("valid timestamp").timestamp();
    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET.as_ref())).map_err(|_| Error::JWTTokenCreationError)
}

fn jwt_from_header(headers: &rocket::http::HeaderMap) -> JWTResult<String> {
    let header = match headers.get_one(AUTHORIZATION.as_str()) {
        Some(header) => header,
        None => return Err(Error::NoAuthHeaderError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(auth_header) => auth_header,
        Err(_) => return Err(Error::InvalidAuthHeaderError),
    };
    if !auth_header.starts_with("FunLe Security") {
        return Err(Error::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches("FunLe Security").to_string())
}

pub async fn authorize(headers: &rocket::http::HeaderMap<'_>) -> WebResult<String> {
    match jwt_from_header(headers) {
        Ok(jwt) => {
            let decode = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(JWT_SECRET.as_ref()),
                &Validation::new(Algorithm::HS512),
            ).map_err(|_| Error::JWTTokenNotValidError).unwrap();

            Ok(decode.claims.sub)
        }
        Err(e) => Err(rocket::http::Status::Unauthorized),
    }
}