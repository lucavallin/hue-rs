use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceGetAllOfMetadata {
    /// Human readable name of a resource
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "archetype", skip_serializing_if = "Option::is_none")]
    pub archetype: Option<models::ProductArchetype>,
}

impl DeviceGetAllOfMetadata {
    pub fn new() -> DeviceGetAllOfMetadata {
        DeviceGetAllOfMetadata {
            name: None,
            archetype: None,
        }
    }
}
