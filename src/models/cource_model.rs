// use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::{
    info_model::InfoModel,
    tests_model::TestModel,
};

// TODO: Rewrite CourseModel

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub levels: Vec<Level>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Level {
    Info (InfoModel),
    Test (TestModel),
}