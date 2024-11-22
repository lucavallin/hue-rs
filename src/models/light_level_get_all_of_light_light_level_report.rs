use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightLevelGetAllOfLightLightLevelReport {
    /// last time the value of this property is changed.
    #[serde(rename = "changed", skip_serializing_if = "Option::is_none")]
    pub changed: Option<String>,
    /// Light level in 10000*log10(lux) +1 measured by sensor. Logarithmic scale used because the human eye adjusts to light levels and small changes at low lux levels are more noticeable than at high lux levels. This allows use of linear scale configuration sliders.
    #[serde(rename = "light_level", skip_serializing_if = "Option::is_none")]
    pub light_level: Option<i32>,
}

impl LightLevelGetAllOfLightLightLevelReport {
    pub fn new() -> LightLevelGetAllOfLightLightLevelReport {
        LightLevelGetAllOfLightLightLevelReport {
            changed: None,
            light_level: None,
        }
    }
}
