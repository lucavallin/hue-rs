use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLightLevels200Response {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<models::Error>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::LightLevelGet>>,
}

impl GetLightLevels200Response {
    pub fn new() -> GetLightLevels200Response {
        GetLightLevels200Response {
            errors: None,
            data: None,
        }
    }
}
