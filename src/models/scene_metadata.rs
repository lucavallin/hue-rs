use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SceneMetadata {
    /// Human readable name of a resource
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<Box<models::ResourceIdentifier>>,
    /// Application specific data. Free format string.
    #[serde(rename = "appdata", skip_serializing_if = "Option::is_none")]
    pub appdata: Option<String>,
}

impl SceneMetadata {
    pub fn new() -> SceneMetadata {
        SceneMetadata {
            name: None,
            image: None,
            appdata: None,
        }
    }
}
