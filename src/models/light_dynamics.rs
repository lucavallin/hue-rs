use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightDynamics {
    /// Duration of a light transition or timed effects in ms.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// Speed of dynamic palette or effect. The speed is valid for the dynamic palette if the status is `dynamic_palette` or for the corresponding effect listed in status. In case of status `none`, the speed is not valid.
    #[serde(rename = "speed", skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
}

impl LightDynamics {
    pub fn new() -> LightDynamics {
        LightDynamics {
            duration: None,
            speed: None,
        }
    }
}
