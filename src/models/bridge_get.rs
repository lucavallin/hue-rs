use crate::models;
use serde::{Deserialize, Serialize};

/// BridgeGet : Definition of a bridge resource
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BridgeGet {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Unique identifier representing a specific resource instance
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Clip v1 resource identifier
    #[serde(rename = "id_v1", skip_serializing_if = "Option::is_none")]
    pub id_v1: Option<String>,
    #[serde(rename = "owner", skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<models::ResourceIdentifier>>,
    /// Unique identifier of the bridge as printed on the device. Lower case (shouldn't it be upper case?)
    #[serde(rename = "bridge_id", skip_serializing_if = "Option::is_none")]
    pub bridge_id: Option<String>,
    #[serde(rename = "time_zone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<Box<models::BridgeGetAllOfTimeZone>>,
}

impl BridgeGet {
    /// Definition of a bridge resource
    pub fn new() -> BridgeGet {
        BridgeGet {
            r#type: None,
            id: None,
            id_v1: None,
            owner: None,
            bridge_id: None,
            time_zone: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "bridge")]
    Bridge,
}

impl Default for Type {
    fn default() -> Type {
        Self::Bridge
    }
}
