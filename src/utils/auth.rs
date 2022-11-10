use chrono::{Duration, Utc};
use jsonwebtoken::TokenData;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::futures::stream::Forward;
use rocket::http::Status;
use rocket::request;
use rocket::response::status::{self, Unauthorized};

use rocket::{
    http::hyper::header::AUTHORIZATION, request::FromRequest, request::Outcome,
    serde::json::Json,
};
use serde::{Deserialize, Serialize};

use super::errors::{Error, Result as JWTResult};
use crate::models::response::Response;
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


#[rocket::async_trait]
impl<'r> FromRequest<'r> for Claims {
    type Error = status::Custom<Json<Response>>;

    async fn from_request(
        request: &'r rocket::request::Request<'_>,
    ) -> request::Outcome<Self, status::Custom<Json<Response>>> {
        let auth_header = request.headers().get_one(AUTHORIZATION.as_str());
        if auth_header.is_none() {
            return Outcome::Failure((
                Status::Unauthorized,
                status::Custom(
                    Status::Unauthorized,
                    Json(Response {
                        status: "error".to_string(),
                        message: "No Authorization header".to_string(),
                        data: None,
                    }),
                ),
            ));
        }

        let auth_header = auth_header.unwrap();
        let token = auth_header.replace("Bearer ", "");
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(JWT_SECRET),
            &Validation::new(Algorithm::HS256),
        );

        if token_data.is_err() {
            return Outcome::Failure((
                Status::Unauthorized,
                status::Custom(
                    Status::Unauthorized,
                    Json(Response {
                        status: "error".to_string(),
                        message: "No Authorization header".to_string(),
                        data: None,
                    }),
                ),
            ));
        }

        let token_data = token_data.unwrap();
        Outcome::Success(token_data.claims)
    }
}

pub fn generate_jwt(user_id: String, role: UserRole) -> JWTResult<String> {
    let now = Utc::now();
    let exp = now + Duration::days(1);
    let now_timestamp = now.timestamp();
    let exp_timestamp = exp.timestamp();

    let claims = Claims {
        sub: user_id,
        iat: now_timestamp,
        exp: exp_timestamp,
        role: role,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    ).unwrap();

    Ok(token)
}

fn decode_token(token: &str) -> JWTResult<Claims> {
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(Algorithm::HS256),
    ).unwrap();

    Ok(token_data.claims)
}

fn verify_token(token_data: &TokenData<Claims>) -> bool {
    let now = Utc::now();
    let now_timestamp = now.timestamp();
    let exp_timestamp = token_data.claims.exp;

    if now_timestamp > exp_timestamp {
        return false;
    }

    true
}
