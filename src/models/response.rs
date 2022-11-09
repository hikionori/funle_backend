use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub status: String,
    pub message: String,
    pub data: Option<Value>
}