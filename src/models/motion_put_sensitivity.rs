use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MotionPutSensitivity {
    /// Sensitivity of the sensor. Value in the range 0 to sensitivity_max.
    #[serde(rename = "sensitivity", skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<i32>,
}

impl MotionPutSensitivity {
    pub fn new() -> MotionPutSensitivity {
        MotionPutSensitivity { sensitivity: None }
    }
}
