use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MotionGetAllOfMotionMotionReport {
    /// last time the value of this property is changed
    #[serde(rename = "changed", skip_serializing_if = "Option::is_none")]
    pub changed: Option<String>,
    /// true if motion is detected
    #[serde(rename = "motion", skip_serializing_if = "Option::is_none")]
    pub motion: Option<bool>,
}

impl MotionGetAllOfMotionMotionReport {
    pub fn new() -> MotionGetAllOfMotionMotionReport {
        MotionGetAllOfMotionMotionReport {
            changed: None,
            motion: None,
        }
    }
}
