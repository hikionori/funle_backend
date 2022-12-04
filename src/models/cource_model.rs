// use std::str::FromStr;
use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// use crate::models::{info_model::InfoModel, tests_model::TestModel};

/*
 * CourseModel
 * {
 *     id: "<cource_id>",
 *     description: "<cource_description>",
 *     levels: {
 *      <level_number>: [{type(<info | test>): {id: "<type_id>"}}, ...]
 *     }
 * }
 */

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    pub levels: HashMap<i32, Vec<Level>>,
}

/* JSON example:
 * {
 *  id: "<level_id>",
 *  title: "<cource_title>",
 *  description: "<cource_description>",
 *  levels: {
 *      1: [{info: {id: "<info_id>", title: "<info_title>", mini_img: <array of bytes>}}, ...],
 *      2: [{test: {id: "<test_id>", title: "<test_title>", mini_img: <array of bytes>}}, ...],
 *      ...
 * }
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    pub id: String,          // id of info or test
    pub title: String,       // title of info or test
    pub mini_image: Vec<u8>, // mini image of info or test
}

impl Level {
    pub fn new(id: String, title: String, mini_image: Vec<u8>) -> Self {
        Self {
            id,
            title,
            mini_image,
        }
    }
}
