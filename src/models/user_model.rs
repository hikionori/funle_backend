use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct UserModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
    pub role: UserRole,
    pub progress: UserProgress,
    // pub friends: Vec<String>, // Means id of friends
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserProgress {
    pub courses: Vec<String>, // id of courses
    pub tests: Vec<String>, // id of tests
    pub infos: Vec<String>, // id of infos
}