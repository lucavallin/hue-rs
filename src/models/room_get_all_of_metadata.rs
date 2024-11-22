use crate::models;
use serde::{Deserialize, Serialize};

/// RoomGetAllOfMetadata : configuration object for a room
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoomGetAllOfMetadata {
    /// Human readable name of a resource
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "archetype", skip_serializing_if = "Option::is_none")]
    pub archetype: Option<models::RoomArchetype>,
}

impl RoomGetAllOfMetadata {
    /// configuration object for a room
    pub fn new() -> RoomGetAllOfMetadata {
        RoomGetAllOfMetadata {
            name: None,
            archetype: None,
        }
    }
}
