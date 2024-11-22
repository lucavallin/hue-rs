use crate::models;
use serde::{Deserialize, Serialize};

/// LightGetAllOfSignaling : Feature containing signaling properties.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LightGetAllOfSignaling {
    #[serde(rename = "signal_values", skip_serializing_if = "Option::is_none")]
    pub signal_values: Option<Vec<models::SupportedSignals>>,
    /// Timestamp indicating when the active signal is expected to end. Value is not set if there is no_signal
    #[serde(rename = "estimated_end", skip_serializing_if = "Option::is_none")]
    pub estimated_end: Option<i32>,
    /// Colors that were provided for the active effect.
    #[serde(rename = "colors", skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<models::Color>>,
}

impl LightGetAllOfSignaling {
    /// Feature containing signaling properties.
    pub fn new() -> LightGetAllOfSignaling {
        LightGetAllOfSignaling {
            signal_values: None,
            estimated_end: None,
            colors: None,
        }
    }
}
