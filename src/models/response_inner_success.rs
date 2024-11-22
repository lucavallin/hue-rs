use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseInnerSuccess {
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "clientkey", skip_serializing_if = "Option::is_none")]
    pub clientkey: Option<String>,
}

impl ResponseInnerSuccess {
    pub fn new() -> ResponseInnerSuccess {
        ResponseInnerSuccess {
            username: None,
            clientkey: None,
        }
    }
}
