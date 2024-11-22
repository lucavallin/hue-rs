use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorTemperatureDelta {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// Mirek delta to current mirek. Clip at mirek_minimum and mirek_maximum of mirek_schema.
    #[serde(rename = "mirek_delta", skip_serializing_if = "Option::is_none")]
    pub mirek_delta: Option<i32>,
}

impl ColorTemperatureDelta {
    pub fn new() -> ColorTemperatureDelta {
        ColorTemperatureDelta {
            action: None,
            mirek_delta: None,
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
