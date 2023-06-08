use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// `UserModel` is a struct that has an `id` field of type `Option<ObjectId>`, a `username` field of
/// type `String`, an `email` field of type `String`, a `hashed_password` field of type `String`, a
/// `role` field of type `UserRole`, a `progress` field of type `UserProgress`, and a `friends` field of
/// type `Vec<String>`.
/// 
/// The `#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
/// 
/// Properties:
/// 
/// * `id`: The id of the user.
/// * `username`: The username of the user.
/// * `email`: The email address of the user.
/// * `hashed_password`: The hashed password of the user.
/// * `role`: This is the role of the user. It can be either a student or a teacher or a just user.
/// * `progress`: This is a struct that contains the user's progress in the game.
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

/// This is a enum that has three variants: `User`, `Student`, and `Teacher`.
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

/// `UserProgress` is a struct that contains a vector of strings for courses, tests, and infos.
/// 
/// Properties:
/// 
/// * `courses`: A list of course ids that the user has completed.
/// * `tests`: A list of tests that the user has completed.
/// * `infos`: A list of info ids that the user has read.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct UserProgress {
    pub courses: Vec<String>, // id of courses
    pub tests: Vec<String>, // id of tests
    pub infos: Vec<String>, // id of infos
    pub nodes: Vec<String>, // id of nodes
}