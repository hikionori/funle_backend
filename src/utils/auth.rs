use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use chrono::{Duration, Utc};
// use time::{OffsetDateTime};

use rocket::http::hyper::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

use super::errors::{Error, Result as JWTResult};
use crate::models::user_model::UserRole;

type WebResult<T> = std::result::Result<T, rocket::http::Status>;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    iat: i64,
    exp: i64,
    role: UserRole,
}

// TODO: Переписать Генерацию токена

const JWT_SECRET: &[u8] = b"secret";

// const ONE_WEEK: i64 = 60 * 60 * 24 * 7;


pub async fn create_access_token(user_id: String, role: UserRole) -> JWTResult<String> {
    let expiration = Utc::now().checked_add_signed(Duration::seconds(60)).expect("valid timestamp").timestamp();
    let claims = Claims {
        sub: user_id,
        iat: Utc::now().timestamp(),
        exp: expiration,
        role,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET.as_ref())).map_err(|_| Error::JWTTokenCreationError)
}

pub async fn create_refresh_token(user_id: String, role: UserRole) -> JWTResult<String> {
    let expiration = Utc::now().checked_add_signed(Duration::days(30)).expect("valid timestamp").timestamp();
    let claims = Claims {
        sub: user_id,
        iat: Utc::now().timestamp(),
        exp: expiration,
        role,
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

fn decode_jwt(token: &str) -> JWTResult<Claims> {
    let validation = Validation::new(Algorithm::HS512);
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(JWT_SECRET.as_ref()), &validation);
    match token_data {
        Ok(data) => Ok(data.claims),
        Err(_) => Err(Error::JWTTokenDecodeError),
    }
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