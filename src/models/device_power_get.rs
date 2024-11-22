use crate::models;
use serde::{Deserialize, Serialize};

/// DevicePowerGet : Definition of a bridge power resource
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DevicePowerGet {
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
    #[serde(rename = "power_state", skip_serializing_if = "Option::is_none")]
    pub power_state: Option<Box<models::DevicePowerGetAllOfPowerState>>,
}

impl DevicePowerGet {
    /// Definition of a bridge power resource
    pub fn new() -> DevicePowerGet {
        DevicePowerGet {
            r#type: None,
            id: None,
            id_v1: None,
            owner: None,
            power_state: None,
        }
    }
}
