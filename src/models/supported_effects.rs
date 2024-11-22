use crate::models;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportedEffects {
    #[serde(rename = "prism")]
    Prism,
    #[serde(rename = "opal")]
    Opal,
    #[serde(rename = "glisten")]
    Glisten,
    #[serde(rename = "sparkle")]
    Sparkle,
    #[serde(rename = "fire")]
    Fire,
    #[serde(rename = "candle")]
    Candle,
    #[serde(rename = "no_effect")]
    NoEffect,
}

impl std::fmt::Display for SupportedEffects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Prism => write!(f, "prism"),
            Self::Opal => write!(f, "opal"),
            Self::Glisten => write!(f, "glisten"),
            Self::Sparkle => write!(f, "sparkle"),
            Self::Fire => write!(f, "fire"),
            Self::Candle => write!(f, "candle"),
            Self::NoEffect => write!(f, "no_effect"),
        }
    }
}

impl Default for SupportedEffects {
    fn default() -> SupportedEffects {
        Self::Prism
    }
}
