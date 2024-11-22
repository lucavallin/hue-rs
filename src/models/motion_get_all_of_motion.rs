use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MotionGetAllOfMotion {
    /// Deprecated. Moved to motion_report/motion.
    #[serde(rename = "motion", skip_serializing_if = "Option::is_none")]
    pub motion: Option<bool>,
    /// Deprecated. Motion is valid when motion_report property is present, invalid when absent.
    #[serde(rename = "motion_valid", skip_serializing_if = "Option::is_none")]
    pub motion_valid: Option<bool>,
    #[serde(rename = "motion_report", skip_serializing_if = "Option::is_none")]
    pub motion_report: Option<Box<models::MotionGetAllOfMotionMotionReport>>,
}

impl MotionGetAllOfMotion {
    pub fn new() -> MotionGetAllOfMotion {
        MotionGetAllOfMotion {
            motion: None,
            motion_valid: None,
            motion_report: None,
        }
    }
}
