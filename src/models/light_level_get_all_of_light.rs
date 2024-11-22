use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightLevelGetAllOfLight {
    /// Deprecated. Moved to light_level_report/light_level
    #[serde(rename = "light_level", skip_serializing_if = "Option::is_none")]
    pub light_level: Option<i32>,
    /// Deprecated. Indication whether the value presented in light_level is valid
    #[serde(rename = "light_level_valid", skip_serializing_if = "Option::is_none")]
    pub light_level_valid: Option<bool>,
    #[serde(rename = "light_level_report", skip_serializing_if = "Option::is_none")]
    pub light_level_report: Option<Box<models::LightLevelGetAllOfLightLightLevelReport>>,
}

impl LightLevelGetAllOfLight {
    pub fn new() -> LightLevelGetAllOfLight {
        LightLevelGetAllOfLight {
            light_level: None,
            light_level_valid: None,
            light_level_report: None,
        }
    }
}
