// use jsonwebtoken::errors::ErrorKind
use thiserror::Error;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("wrong credentials")]
    WrongCredentialsError,
    #[error("jwt token not valid")]
    JWTTokenNotValidError,
    #[error("jwt token creation error")]
    JWTTokenCreationError,
    #[error("no auth header")]
    NoAuthHeaderError,
    #[error("invalid auth header")]
    InvalidAuthHeaderError,
    #[error("token decode error")]
    JWTTokenDecodeError
}

#[derive(Debug, Error)]
pub enum TestsError {
    #[error("we are cant get tests")]
    WeAreCanNotGetTests,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub status: u16,
}