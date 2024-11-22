use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TemperatureGet {
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
    /// `true` when sensor is activated, `false` when deactivated
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "temperature", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Box<models::TemperatureGetAllOfTemperature>>,
}

impl TemperatureGet {
    pub fn new() -> TemperatureGet {
        TemperatureGet {
            r#type: None,
            id: None,
            id_v1: None,
            owner: None,
            enabled: None,
            temperature: None,
        }
    }
}
