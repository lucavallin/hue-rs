use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightGetAllOfDimming {
    /// Brightness percentage. value cannot be 0, writing 0 changes it to lowest possible brightness
    #[serde(rename = "brightness", skip_serializing_if = "Option::is_none")]
    pub brightness: Option<f64>,
    /// Percentage of the maximum lumen the device outputs on minimum brightness
    #[serde(rename = "min_dim_level", skip_serializing_if = "Option::is_none")]
    pub min_dim_level: Option<f64>,
}

impl LightGetAllOfDimming {
    pub fn new() -> LightGetAllOfDimming {
        LightGetAllOfDimming {
            brightness: None,
            min_dim_level: None,
        }
    }
}
