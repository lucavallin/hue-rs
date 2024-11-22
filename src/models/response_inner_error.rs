use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseInnerError {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ResponseInnerError {
    pub fn new() -> ResponseInnerError {
        ResponseInnerError {
            r#type: None,
            address: None,
            description: None,
        }
    }
}
