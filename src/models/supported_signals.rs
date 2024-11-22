use crate::models;
use serde::{Deserialize, Serialize};

/// SupportedSignals : Indicates which signal is currently active.
/// Indicates which signal is currently active.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportedSignals {
    #[serde(rename = "no_signal")]
    NoSignal,
    #[serde(rename = "on_off")]
    OnOff,
    #[serde(rename = "on_off_color")]
    OnOffColor,
    #[serde(rename = "alternating")]
    Alternating,
}

impl std::fmt::Display for SupportedSignals {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoSignal => write!(f, "no_signal"),
            Self::OnOff => write!(f, "on_off"),
            Self::OnOffColor => write!(f, "on_off_color"),
            Self::Alternating => write!(f, "alternating"),
        }
    }
}

impl Default for SupportedSignals {
    fn default() -> SupportedSignals {
        Self::NoSignal
    }
}
