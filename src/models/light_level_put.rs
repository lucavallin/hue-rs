use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightLevelPut {
    /// Type of the supported resources (always `light_level` here)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// true when sensor is activated, false when deactivated
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl LightLevelPut {
    pub fn new() -> LightLevelPut {
        LightLevelPut {
            r#type: None,
            enabled: None,
        }
    }
}
