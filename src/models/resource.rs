use crate::models;
use serde::{Deserialize, Serialize};

/// Resource : Common resource properties
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    /// Type of the supported resources
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Unique identifier representing a specific resource instance
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Clip v1 resource identifier
    #[serde(rename = "id_v1", skip_serializing_if = "Option::is_none")]
    pub id_v1: Option<String>,
}

impl Resource {
    /// Common resource properties
    pub fn new() -> Resource {
        Resource {
            r#type: None,
            id: None,
            id_v1: None,
        }
    }
}
