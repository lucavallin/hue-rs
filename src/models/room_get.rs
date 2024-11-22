use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RoomGet {
    /// Type of the supported resources
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Unique identifier representing a specific resource instance
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Clip v1 resource identifier
    #[serde(rename = "id_v1", skip_serializing_if = "Option::is_none")]
    pub id_v1: Option<String>,
    /// Child devices/services to group by the derived group
    #[serde(rename = "children", skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<models::ResourceIdentifier>>,
    /// References all services aggregating control and state of children in the group. This includes all services grouped in the group hierarchy given by child relation. This includes all services of a device grouped in the group hierarchy given by child relation. Aggregation is per service type, ie every service type which can be grouped has a corresponding definition of grouped type. Supported types: – grouped_light
    #[serde(rename = "services", skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<models::ResourceIdentifier>>,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::RoomGetAllOfMetadata>>,
}

impl RoomGet {
    pub fn new() -> RoomGet {
        RoomGet {
            r#type: None,
            id: None,
            id_v1: None,
            children: None,
            services: None,
            metadata: None,
        }
    }
}
