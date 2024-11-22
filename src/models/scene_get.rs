use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SceneGet {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Unique identifier representing a specific resource instance
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Clip v1 resource identifier
    #[serde(rename = "id_v1", skip_serializing_if = "Option::is_none")]
    pub id_v1: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<models::ResourceIdentifier>>,
    /// List of actions to be executed synchronously on recall
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<models::ActionGet>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::SceneMetadata>>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<Box<models::ResourceIdentifier>>,
    #[serde(rename = "palette", skip_serializing_if = "Option::is_none")]
    pub palette: Option<Box<models::ScenePalette>>,
    /// Speed of dynamic palette for this scene
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    /// Indicates whether to automatically start the scene dynamically on active recall
    #[serde(rename = "auto_dynamic", skip_serializing_if = "Option::is_none")]
    pub auto_dynamic: Option<bool>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<models::SceneGetAllOfStatus>>,
}

impl SceneGet {
    pub fn new() -> SceneGet {
        SceneGet {
            r#type: None,
            id: None,
            id_v1: None,
            owner: None,
            actions: None,
            metadata: None,
            group: None,
            palette: None,
            speed: None,
            auto_dynamic: None,
            status: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "scene")]
    Scene,
}

impl Default for Type {
    fn default() -> Type {
        Self::Scene
    }
}
