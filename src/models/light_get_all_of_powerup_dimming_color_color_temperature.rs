use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightGetAllOfPowerupDimmingColorColorTemperature {
    /// color temperature in mirek or null when the light color is not in the ct spectrum
    #[serde(rename = "mirek", skip_serializing_if = "Option::is_none")]
    pub mirek: Option<i32>,
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Box<models::Color>>,
}

impl LightGetAllOfPowerupDimmingColorColorTemperature {
    pub fn new() -> LightGetAllOfPowerupDimmingColorColorTemperature {
        LightGetAllOfPowerupDimmingColorColorTemperature {
            mirek: None,
            color: None,
        }
    }
}
