use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SceneRecall {
    /// When writing active, the actions in the scene are executed on the target. dynamic_palette starts dynamic scene with colors in the Palette object.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// Transition to the scene within the timeframe given by duration
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "dimming", skip_serializing_if = "Option::is_none")]
    pub dimming: Option<Box<models::Dimming>>,
}

impl SceneRecall {
    pub fn new() -> SceneRecall {
        SceneRecall {
            action: None,
            duration: None,
            dimming: None,
        }
    }
}
/// When writing active, the actions in the scene are executed on the target. dynamic_palette starts dynamic scene with colors in the Palette object.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "dynamic_palette")]
    DynamicPalette,
    #[serde(rename = "static")]
    Static,
}

impl Default for Action {
    fn default() -> Action {
        Self::Active
    }
}
