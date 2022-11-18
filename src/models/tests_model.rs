#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_parens)]

use serde::{Deserialize, Serialize};
use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
// TODO: Create tests model

#[derive(Debug, Serialize, Deserialize)]
pub struct TestModel {
    pub id: Option<ObjectId>,
    pub text_of_question: String,
    pub answers: Vec<String>,
    pub correct_answer: String,
}