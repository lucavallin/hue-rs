use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartSceneMetadataWithImage {
    /// Human readable name of a resource
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Application specific data. Free format string.
    #[serde(rename = "appdata", skip_serializing_if = "Option::is_none")]
    pub appdata: Option<String>,
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<Box<models::ResourceIdentifier>>,
}

impl SmartSceneMetadataWithImage {
    pub fn new() -> SmartSceneMetadataWithImage {
        SmartSceneMetadataWithImage {
            name: None,
            appdata: None,
            image: None,
        }
    }
}
