use crate::models;
use serde::{Deserialize, Serialize};

/// ResourceOwned : Common resource properties including the owner
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceOwned {
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
}

impl ResourceOwned {
    /// Common resource properties including the owner
    pub fn new() -> ResourceOwned {
        ResourceOwned {
            r#type: None,
            id: None,
            id_v1: None,
            owner: None,
        }
    }
}
