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
    #[error("we are cant create test")]
    WeAreCanNotCreateTest,
    #[error("we are cant update test")]
    WeAreCanNotUpdateTest,
    #[error("we are cant delete test")]
    WeAreCanNotDeleteTest,
    #[error("we are cant get test")]
    WeAreCanNotGetTest
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error("We are cant create user")]
    WeAreCanNotCreateUser,
    #[error("We are cant get user")]
    WeAreCanNotGetUser,
    #[error("We are cant update user")]
    WeAreCanNotUpdateUser,
    #[error("We are cant delete user")]
    WeAreCanNotDeleteUser,
}


#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub status: u16,
}