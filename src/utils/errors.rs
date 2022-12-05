// use jsonwebtoken::errors::ErrorKind
use thiserror::Error;
use serde::{Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("jwt token not valid")]
    JWTTokenNotValid,
    #[error("wrong credentials")]
    WrongCredentials,
    #[error("jwt token creation error")]
    JWTTokenCreation,
    #[error("no auth header")]
    NoAuthHeader,
    #[error("invalid auth header")]
    InvalidAuthHeader,
    #[error("token decode error")]
    JWTTokenDecode
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