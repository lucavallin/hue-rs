use crate::models;
use serde::{Deserialize, Serialize};

/// GroupedLightGetAllOfAlert : Joined alert control
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupedLightGetAllOfAlert {
    #[serde(rename = "action_values", skip_serializing_if = "Option::is_none")]
    pub action_values: Option<Vec<String>>,
}

impl GroupedLightGetAllOfAlert {
    /// Joined alert control
    pub fn new() -> GroupedLightGetAllOfAlert {
        GroupedLightGetAllOfAlert {
            action_values: None,
        }
    }
}
