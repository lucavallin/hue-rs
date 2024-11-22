use crate::models;
use serde::{Deserialize, Serialize};

/// GroupedLightGetAllOfSignaling : Feature containing basic signaling properties.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupedLightGetAllOfSignaling {
    /// Signals that the light supports.
    #[serde(rename = "signal_values", skip_serializing_if = "Option::is_none")]
    pub signal_values: Option<Vec<models::SupportedSignals>>,
}

impl GroupedLightGetAllOfSignaling {
    /// Feature containing basic signaling properties.
    pub fn new() -> GroupedLightGetAllOfSignaling {
        GroupedLightGetAllOfSignaling {
            signal_values: None,
        }
    }
}
