use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartSceneMetadata {
    /// Human readable name of a resource
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application specific data. Free format string.
    #[serde(rename = "appdata", skip_serializing_if = "Option::is_none")]
    pub appdata: Option<String>,
}

impl SmartSceneMetadata {
    pub fn new() -> SmartSceneMetadata {
        SmartSceneMetadata {
            name: None,
            appdata: None,
        }
    }
}
