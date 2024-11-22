use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScenePut {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// List of actions to be executed synchronously on recall
    #[serde(rename = "actions", skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<models::ActionPost>>,
    #[serde(rename = "recall", skip_serializing_if = "Option::is_none")]
    pub recall: Option<Box<models::SceneRecall>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::SceneMetadata>>,
    #[serde(rename = "palette", skip_serializing_if = "Option::is_none")]
    pub palette: Option<Box<models::ScenePalette>>,
    /// Speed of dynamic palette for this scene
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    /// Indicates whether to automatically start the scene dynamically on active recall
    #[serde(rename = "auto_dynamic", skip_serializing_if = "Option::is_none")]
    pub auto_dynamic: Option<bool>,
}

impl ScenePut {
    pub fn new() -> ScenePut {
        ScenePut {
            r#type: None,
            actions: None,
            recall: None,
            metadata: None,
            palette: None,
            speed: None,
            auto_dynamic: None,
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
