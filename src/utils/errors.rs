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

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub status: u16,
}