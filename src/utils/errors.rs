// use jsonwebtoken::errors::ErrorKind
use thiserror::Error;
use serde::{Serialize};

pub type Result<T> = std::result::Result<T, Error>;

/// A custom error type for JWT.
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

/// Creating a custom error type for the tests.
#[derive(Debug, Error)]
pub enum TestsError {
    #[error("we are cant get tests")]
    GetTests,
    #[error("we are cant create test")]
    CreateTest,
    #[error("we are cant update test")]
    UpdateTest,
    #[error("we are cant delete test")]
    DeleteTest,
    #[error("we are cant get test")]
    GetTest
}

/// Creating a custom error type for the user.
#[derive(Debug, Error)]
pub enum UserError {
    #[error("We are cant create user")]
    CreateUser,
    #[error("We are cant get user")]
    GetUser,
    #[error("We are cant update user")]
    UpdateUser,
    #[error("We are cant delete user")]
    DeleteUser,
}

/// Creating a custom error type for the infos.
#[derive(Debug, Error)]
pub enum InfosError {
    #[error("We are cant create info")]
    CreateInfo,
    #[error("We are cant get info")]
    GetInfo,
    #[error("We are cant update info")]
    UpdateInfo,
    #[error("We are cant delete info")]
    DeleteInfo,
    #[error("we are cant get infos")]
    GetInfos

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