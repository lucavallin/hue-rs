use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MotionPut {
    /// Type of the supported resources (always `motion` here)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// true when the sensor is activated, false when deactivated
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "sensitivity", skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<Box<models::MotionPutSensitivity>>,
}

impl MotionPut {
    pub fn new() -> MotionPut {
        MotionPut {
            r#type: None,
            enabled: None,
            sensitivity: None,
        }
    }
}
