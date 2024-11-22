use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DimmingDelta {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// Brightness percentage of full-scale increase delta to current dimlevel. Clip at Max-level or Min-level.
    #[serde(rename = "brightness_delta", skip_serializing_if = "Option::is_none")]
    pub brightness_delta: Option<f64>,
}

impl DimmingDelta {
    pub fn new() -> DimmingDelta {
        DimmingDelta {
            action: None,
            brightness_delta: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "stop")]
    Stop,
}

impl Default for Action {
    fn default() -> Action {
        Self::Up
    }
}
