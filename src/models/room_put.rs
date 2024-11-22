use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoomPut {
    /// Type of the supported resources (always `room` here)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Child devices/services to group by the derived group
    #[serde(rename = "children", skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<models::ResourceIdentifier>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::RoomGetAllOfMetadata>>,
}

impl RoomPut {
    pub fn new() -> RoomPut {
        RoomPut {
            r#type: None,
            children: None,
            metadata: None,
        }
    }
}
