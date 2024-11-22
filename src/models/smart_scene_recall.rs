use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmartSceneRecall {
    /// Activate will start the smart (24h) scene; deactivate will stop it
    #[serde(rename = "action")]
    pub action: Action,
}

impl SmartSceneRecall {
    pub fn new(action: Action) -> SmartSceneRecall {
        SmartSceneRecall { action }
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
