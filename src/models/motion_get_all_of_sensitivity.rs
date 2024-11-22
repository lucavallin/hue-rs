use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MotionGetAllOfSensitivity {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Sensitivity of the sensor. Value in the range 0 to sensitivity_max
    #[serde(rename = "sensitivity", skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<i32>,
    /// Maximum value of the sensitivity configuration attribute.
    #[serde(rename = "sensitivity_max", skip_serializing_if = "Option::is_none")]
    pub sensitivity_max: Option<i32>,
}

impl MotionGetAllOfSensitivity {
    pub fn new() -> MotionGetAllOfSensitivity {
        MotionGetAllOfSensitivity {
            status: None,
            sensitivity: None,
            sensitivity_max: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "changing")]
    Changing,
}

impl Default for Status {
    fn default() -> Status {
        Self::Set
    }
}
