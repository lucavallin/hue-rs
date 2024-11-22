use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightGetAllOfDynamics {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::SupportedDynamicStatus>,
    /// Statuses in which a lamp could be when playing dynamics.
    #[serde(rename = "status_values", skip_serializing_if = "Option::is_none")]
    pub status_values: Option<Vec<models::SupportedDynamicStatus>>,
    /// speed of dynamic palette or effect. The speed is valid for the dynamic palette if the status is dynamic_palette or for the corresponding effect listed in status. In case of status none, the speed is not valid
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    /// Indicates whether the value presented in speed is valid
    #[serde(rename = "speed_valid", skip_serializing_if = "Option::is_none")]
    pub speed_valid: Option<bool>,
}

impl LightGetAllOfDynamics {
    pub fn new() -> LightGetAllOfDynamics {
        LightGetAllOfDynamics {
            status: None,
            status_values: None,
            speed: None,
            speed_valid: None,
        }
    }
}
