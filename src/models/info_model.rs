// use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// TODO: Create a info model

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub content: Vec<String>
}