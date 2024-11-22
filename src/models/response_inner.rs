use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseInner {
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<Box<models::ResponseInnerSuccess>>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::ResponseInnerError>>,
}

impl ResponseInner {
    pub fn new() -> ResponseInner {
        ResponseInner {
            success: None,
            error: None,
        }
    }
}
