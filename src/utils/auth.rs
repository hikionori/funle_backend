#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(unused_variables)]

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
// use time::{OffsetDateTime};

use rocket::State;
use rocket::http::hyper::header::AUTHORIZATION;
use serde::{Deserialize, Serialize};

use super::errors::{Error, Result as JWTResult};
use crate::models::user_model::UserRole;
use crate::repository::user_repo::UserRepo;

type WebResult<T> = std::result::Result<T, rocket::http::Status>;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String, // user id
    iat: i64, // issued at
    exp: i64, // expiration
    role: UserRole, // role
}

static JWT_SECRET: &[u8] = b"MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDCYxcLzAo66NVlv8tCdzFEY5ap
q9j4xLPHBsan2etNByDWjjCTP31l/XklO6d1FfINng7vOltFmJuzyW6CwOWMC3+6
qp9O6sp3eOPYoxwHu+fayB1BbacR1+K4CkRNyChSOPfjCsbqMGxJ/U+HMB3MQ7fz
xbruxNYCFBW8QkKh1QIDAQAB";


pub async fn create_access_token(user_id: String, role: UserRole) -> JWTResult<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: user_id,
        iat: Utc::now().timestamp(),
        exp: expiration,
        role,
    };
    let header = Header::new(Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
    .map_err(|_| Error::JWTTokenCreationError)
}

pub async fn create_refresh_token(user_id: String, role: UserRole) -> JWTResult<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(30))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: user_id,
        iat: Utc::now().timestamp(),
        exp: expiration,
        role,
    };
    let header = Header::new(Algorithm::HS512);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
    .map_err(|_| Error::JWTTokenCreationError)
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
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &validation,
    );
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
            )
            .map_err(|_| Error::JWTTokenNotValidError)
            .unwrap();

            Ok(decode.claims.sub)
        }
        Err(e) => Err(rocket::http::Status::Unauthorized),
    }
}

pub async fn authorize_token(token: String, db: &State<UserRepo>) -> bool {
    let decode = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| Error::JWTTokenNotValidError)
    .unwrap();

    let user = db.get_user_by_id(decode.claims.sub).await.unwrap();
    if user.is_some() {
        return true;
    }
    false
}
