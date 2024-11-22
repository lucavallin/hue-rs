use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetSmartScenes200Response {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<models::Error>>,
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::SmartSceneGet>>,
}

impl GetSmartScenes200Response {
    pub fn new() -> GetSmartScenes200Response {
        GetSmartScenes200Response {
            errors: None,
            data: None,
        }
    }
}
