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
impl TestModel {
    pub(crate) fn clone(&self) -> TestModel {
        TestModel {
            id: self.id.clone(),
            text_of_question: self.text_of_question.clone(),
            answers: self.answers.clone(),
            correct_answer: self.correct_answer.clone(),
        }
    }
}