use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::tests_model::TestModel;

// TODO: Create a course model

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseModel {
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: String,
    pub tests: Vec<TestModel>,
    // pub infos: Vec<InfoModel>,
    // late add
}