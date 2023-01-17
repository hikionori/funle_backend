// use std::str::FromStr;

use std::collections::HashMap;

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// `InfoModel` is a struct with three fields, `id`, `title`, and `content_levels`. The `id` field is an
/// `Option<ObjectId>`, which means it can be `None` or `Some(ObjectId)`. The `title` field is a
/// `String`. The `content_levels` field is a `HashMap<i32, Vec<ContentLevel>>`.
///
/// The `#[serde_as]` attribute is a custom attribute that tells Serde to use the `as` field to
/// deserialize the type.
///
/// Properties:
///
/// * `id`: The id of the info model.
/// * `title`: The title of the info model.
/// * `content_levels`: A HashMap of the form `HashMap<i32, Vec<ContentLevel>>` where the key is the
/// level of the content and the value is a vector of ContentLevels.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct InfoModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub theme: String,
    #[serde_as(as = "Vec<(_, _)>")]
    pub content_levels: HashMap<i32, Vec<ContentLevel>>,
}

/// `ContentLevel` is a struct that contains a `String` and a `Vec<u8>`.
///
/// The `#[serde_as]` attribute is a custom attribute that tells the `serde` library to use the
/// `serde_as` crate to serialize and deserialize this type.
///
/// The `#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]` attribute is a standard `serde`
/// attribute that tells the `serde` library to automatically generate the code needed to
///
/// Properties:
///
/// * `content_type`: The content type of the data.
/// * `data`: The actual data of the content level in base64.
#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ContentLevel {
    pub content_type: String,
    pub data: String,
}
