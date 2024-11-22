use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<models::Error>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<serde_json::Value>>,
}

impl ApiResponse {
    pub fn new() -> ApiResponse {
        ApiResponse {
            errors: None,
            data: None,
        }
    }
}
