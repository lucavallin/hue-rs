use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MotionGet {
    /// Type of the supported resources
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Unique identifier representing a specific resource instance
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Clip v1 resource identifier
    #[serde(rename = "id_v1", skip_serializing_if = "Option::is_none")]
    pub id_v1: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<models::ResourceIdentifier>>,
    /// ture when the sensor is activated, false when deactivated
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "motion", skip_serializing_if = "Option::is_none")]
    pub motion: Option<Box<models::MotionGetAllOfMotion>>,
    #[serde(rename = "sensitivity", skip_serializing_if = "Option::is_none")]
    pub sensitivity: Option<Box<models::MotionGetAllOfSensitivity>>,
}

impl MotionGet {
    pub fn new() -> MotionGet {
        MotionGet {
            r#type: None,
            id: None,
            id_v1: None,
            owner: None,
            enabled: None,
            motion: None,
            sensitivity: None,
        }
    }
}
