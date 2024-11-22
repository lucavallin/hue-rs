use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupedLightGet {
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
    #[serde(rename = "on", skip_serializing_if = "Option::is_none")]
    pub on: Option<Box<models::On>>,
    #[serde(rename = "dimming", skip_serializing_if = "Option::is_none")]
    pub dimming: Option<Box<models::Dimming>>,
    #[serde(rename = "alert", skip_serializing_if = "Option::is_none")]
    pub alert: Option<Box<models::GroupedLightGetAllOfAlert>>,
    #[serde(rename = "signaling", skip_serializing_if = "Option::is_none")]
    pub signaling: Option<Box<models::GroupedLightGetAllOfSignaling>>,
}

impl GroupedLightGet {
    pub fn new() -> GroupedLightGet {
        GroupedLightGet {
            r#type: None,
            id: None,
            id_v1: None,
            owner: None,
            on: None,
            dimming: None,
            alert: None,
            signaling: None,
        }
    }
}
