use crate::models;
use serde::{Deserialize, Serialize};

/// SupportedDynamicStatus : Current status of the lamp with dynamics.
/// Current status of the lamp with dynamics.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportedDynamicStatus {
    #[serde(rename = "dynamic_palette")]
    DynamicPalette,
    #[serde(rename = "none")]
    None,
}

impl std::fmt::Display for SupportedDynamicStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::DynamicPalette => write!(f, "dynamic_palette"),
            Self::None => write!(f, "none"),
        }
    }
}

impl Default for SupportedDynamicStatus {
    fn default() -> SupportedDynamicStatus {
        Self::DynamicPalette
    }
}
