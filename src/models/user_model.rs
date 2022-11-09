use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// TODO: Переписать модель пользователя

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub role: UserRole
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum UserRole {
    User,
    Student,
    Teacher,
}

impl FromStr for UserRole {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "User" => Ok(UserRole::User),
            "Student" => Ok(UserRole::Student),
            "Teacher" => Ok(UserRole::Teacher),
            _ => Err(()),
        }
    }
}