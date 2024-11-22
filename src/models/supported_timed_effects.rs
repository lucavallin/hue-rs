use crate::models;
use serde::{Deserialize, Serialize};

/// SupportedTimedEffects : Current status values the light is in regarding timed effects
/// Current status values the light is in regarding timed effects
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportedTimedEffects {
    #[serde(rename = "sunrise")]
    Sunrise,
    #[serde(rename = "no_effect")]
    NoEffect,
}

impl std::fmt::Display for SupportedTimedEffects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Sunrise => write!(f, "sunrise"),
            Self::NoEffect => write!(f, "no_effect"),
        }
    }
}

impl Default for SupportedTimedEffects {
    fn default() -> SupportedTimedEffects {
        Self::Sunrise
    }
}
