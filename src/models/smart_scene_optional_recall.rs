use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartSceneOptionalRecall {
    /// Activate will start the smart (24h) scene; deactivate will stop it
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
}

impl SmartSceneOptionalRecall {
    pub fn new() -> SmartSceneOptionalRecall {
        SmartSceneOptionalRecall { action: None }
    }
}
/// Activate will start the smart (24h) scene; deactivate will stop it
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "activate")]
    Activate,
    #[serde(rename = "deactivate")]
    Deactivate,
}

impl Default for Action {
    fn default() -> Action {
        Self::Activate
    }
}
