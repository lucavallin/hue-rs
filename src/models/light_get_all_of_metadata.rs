use crate::models;
use serde::{Deserialize, Serialize};

/// LightGetAllOfMetadata : Deprecated, use metadata on device level
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightGetAllOfMetadata {
    /// Human readable name of a resource
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "archetype", skip_serializing_if = "Option::is_none")]
    pub archetype: Option<models::LightArchetype>,
    /// A fixed mired value of the white lamp
    #[serde(rename = "fixed_mired", skip_serializing_if = "Option::is_none")]
    pub fixed_mired: Option<i32>,
}

impl LightGetAllOfMetadata {
    /// Deprecated, use metadata on device level
    pub fn new() -> LightGetAllOfMetadata {
        LightGetAllOfMetadata {
            name: None,
            archetype: None,
            fixed_mired: None,
        }
    }
}
