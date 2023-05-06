// use jsonwebtoken::errors::ErrorKind
use thiserror::Error;
use serde::{Serialize};

pub type Result<T> = std::result::Result<T, Error>;

/// A custom error type for JWT.
#[allow(unused)]
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


/// `ErrorResponse` is a struct that contains two fields, `error` and `status`, both of which are
/// `String`s.
/// 
/// The `#[derive(Debug, Serialize)]` line is a Rust annotation. It tells the Rust compiler to
/// automatically generate a `Debug` implementation for the `ErrorResponse` type. The `Debug`
/// implementation is used for printing the type to the console. The `Serialize` annotation tells the
/// Rust compiler to automatically generate a `Serialize` implementation for the `ErrorResponse` type.
/// The `Serialize` implementation is used
/// 
/// Properties:
/// 
/// * `error`: The error message that will be displayed to the user.
/// * `status`: The HTTP status code that should be returned to the client.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub status: u16,
}