// use std::str::FromStr;
use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use uuid::Uuid;
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

/// `CourseModel` is a struct with an optional `id` field of type `ObjectId`, a `title` field of type
/// `String`, a `description` field of type `String`, and a `levels` field of type `HashMap<i32,
/// Vec<Level>>`.
///
/// The `#[serde_as]` attribute is a custom attribute that tells `serde` to use the `serde_as` crate to
/// serialize and deserialize the type.
///
/// The `#[serde(rename = "_id", skip_
///
/// Properties:
///
/// * `id`: The id of the course.
/// * `title`: The title of the course.
/// * `description`: A description of the course.
/// * `levels`: HashMap<i32, Vec<Level>>,
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CourseModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: String,
    #[serde_as(as = "Vec<(_, _)>")]
    pub levels: HashMap<i32, Vec<Level>>,
}

/* JSON example:
 * {
 *  id: "<level_id>",
 *  title: "<cource_title>",
 *  description: "<cource_description>",
 *  levels: {
 *      1: [{info: {id: "<info_id>",ids: [...], title: "<info_title>", mini_img: <url to img>}}, ...],
 *      2: [{test: {id: "<test_id>",ids: [...], title: "<test_title>", mini_img: <url to img>}}, ...],
 *      ...
 * }
*/

/// `Level` is a struct that contains an id, a title, and a mini image.
///
/// Properties:
///
/// * `id`: The id of the level.
/// * `ids`: The ids of the content in the level.
/// * `title`: The title of the level.
/// * `mini_image`: The image that will be displayed in the level selection screen.
/// * `type_`: The type of the level (info or test).
/// * `n_of_tests`: The number of tests in the level.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Level {
    pub id: String,              // UUID of level
    pub ids: Vec<String>,        // content ids
    pub title: String,           // title of info or test
    pub mini_image: String,      // mini image of info or test, url
    pub mini_image_success: String, // mini image of info or test, url
    pub type_: String,           // type of level (info or test)
    pub n_of_tests: Option<i32>, // number of tests in the level
}

#[allow(dead_code)]
impl Level {
    pub fn new(
        ids: Vec<String>,
        title: String,
        mini_image: String,
        mini_image_success: String,
        n_of_tests: Option<i32>,
        type_: String,
    ) -> (Self, String) {
        let id = Uuid::new_v4().to_string();
        (
            Self {
                id: id.clone(),
                ids,
                title,
                mini_image,
                mini_image_success,
                type_,
                n_of_tests,
            },
            id,
        )
    }

    pub fn copy(&self) -> Self {
        Self {
            id: self.id.clone(),
            ids: self.ids.clone(),
            title: self.title.clone(),
            mini_image: self.mini_image.clone(),
            mini_image_success: self.mini_image_success.clone(),
            type_: self.type_.clone(),
            n_of_tests: self.n_of_tests,
        }
    }
}
