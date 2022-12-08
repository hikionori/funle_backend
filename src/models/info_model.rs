// use std::str::FromStr;

use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct InfoModel {
//     #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
//     pub id: Option<ObjectId>,
//     pub title: String,
//     pub content: Vec<String>
// }

// ? Maybe it's better to use this models:
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InfoModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub content_levels: HashMap<i32, Vec<ContentLevel>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ContentLevel {
    pub content_type: String,
    pub data: Vec<u8>
}