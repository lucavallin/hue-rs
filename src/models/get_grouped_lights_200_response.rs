use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetGroupedLights200Response {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<models::Error>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::GroupedLightGet>>,
}

impl GetGroupedLights200Response {
    pub fn new() -> GetGroupedLights200Response {
        GetGroupedLights200Response {
            errors: None,
            data: None,
        }
    }
}
