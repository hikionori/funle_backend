#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_parens)]

use serde::{Deserialize, Serialize};
use std::str::FromStr;

use mongodb::{bson::oid::ObjectId, options::UpdateModifications};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub text_of_question: String,
    pub answers: Vec<String>,
    pub correct_answer: String,
    pub level: i32, // mean difficulty 1-5
}

impl TestModel {
    pub(crate) fn clone(&self) -> TestModel {
        TestModel {
            id: self.id,
            text_of_question: self.text_of_question.clone(),
            answers: self.answers.clone(),
            correct_answer: self.correct_answer.clone(),
            level: self.level,
        }
    }
}