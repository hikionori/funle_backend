#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_parens)]

use serde::{Deserialize, Serialize};
use std::str::FromStr;

use mongodb::{bson::oid::ObjectId, options::UpdateModifications};

/// `TestModel` is a struct that has an `id` field of type `Option<ObjectId>`, a `text_of_question`
/// field of type `String`, an `answers` field of type `Vec<String>`, a `correct_answer` field of type
/// `String`, and a `level` field of type `i32`.
/// 
/// The `#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]` line is a Rust annotation that
/// tells the Rust compiler to automatically
/// 
/// Properties:
/// 
/// * `id`: The id of the test.
/// * `text_of_question`: The question itself
/// * `answers`: A vector of strings that are the possible answers to the question.
/// * `correct_answer`: The correct answer to the question
/// * `level`: 1-5, 1 being the easiest and 5 being the hardest
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TestModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub text_of_question: String,
    pub answers: Vec<String>,
    pub correct_answer: String,
    pub level: i32, // mean difficulty 1-5
}

/// `TestModelWithActions` is a struct that has an `id` field of type `Option<ObjectId>`, a `example`
/// field of type `String`, an `actions` field of type `Vec<String>`, an `answer` field of type
/// `String`, and a `level` field of type `i32`.
/// 
/// The `#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]` line is a Rust annotation that
/// tells the Rust compiler to automatically generate some code for us. In this
/// 
/// Properties:
/// 
/// * `id`: The id of the test model.
/// * `example`: The example sentence that the user will see.
/// * `actions`: A list of actions that the user can take.
/// * `answer`: The correct answer to the question
/// * `level`: The difficulty of the question.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TestModelWithActions {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub example: String,
    pub actions: Vec<String>,
    pub answer: String,
    pub level: i32, // mean difficulty 1-5
}